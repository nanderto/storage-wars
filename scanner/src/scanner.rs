//! Public scanning API.

use crate::messages::ScanMessage;
use crate::models::DirEntry;
use crate::worker::{run_worker, SharedQueue, WorkQueue};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};

/// Maximum number of worker threads spawned by [`scan_dir_incremental`].
const MAX_WORKERS: usize = 8;

// ── Public API ────────────────────────────────────────────────────────────────

/// Spawns up to [`MAX_WORKERS`] worker threads that pull directories from a
/// shared work queue protected by a [`Condvar`].
///
/// Results are sent as [`ScanMessage`] values over `tx`.  A final
/// [`ScanMessage::Complete`] is always sent when scanning finishes (even if
/// cancelled).
///
/// # Cancellation
///
/// Set `cancelled` to `true` at any time to request early termination.
/// Workers check the flag before processing each item and before each child.
pub fn scan_dir_incremental(
    root: PathBuf,
    tx: std::sync::mpsc::Sender<ScanMessage>,
    cancelled: Arc<AtomicBool>,
) {
    let queue: SharedQueue = Arc::new((
        Mutex::new(WorkQueue::new(root)),
        Condvar::new(),
    ));

    let worker_count = MAX_WORKERS.min(num_cpus());
    let mut handles = Vec::with_capacity(worker_count);

    for _ in 0..worker_count {
        let q = Arc::clone(&queue);
        let tx_clone = tx.clone();
        let cancelled_clone = Arc::clone(&cancelled);

        let handle = std::thread::spawn(move || {
            run_worker(q, tx_clone, cancelled_clone);
        });

        handles.push(handle);
    }

    // Wait for all workers to finish.
    for handle in handles {
        let _ = handle.join();
    }

    let _ = tx.send(ScanMessage::Complete);
}

/// Reads a single directory level and returns its immediate children.
///
/// Does **not** recurse into subdirectories.  Silently skips entries that
/// produce permission errors.
///
/// # Errors
///
/// Returns an `Err` if the root itself cannot be read (excluding permission
/// errors, which return an empty `Vec`).
pub fn read_dir_immediate(root: &Path) -> std::io::Result<Vec<DirEntry>> {
    let mut entries = Vec::new();

    let read_dir = match std::fs::read_dir(root) {
        Ok(rd) => rd,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Ok(entries);
        }
        Err(e) => return Err(e),
    };

    for result in read_dir {
        let dir_entry = match result {
            Ok(de) => de,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => continue,
            Err(e) => return Err(e),
        };

        let child_path = dir_entry.path();

        let meta = match dir_entry.metadata() {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => continue,
            Err(e) => return Err(e),
        };

        let mut entry = DirEntry::new(child_path, 1);
        if !meta.is_dir() {
            entry.size = meta.len();
        }
        entry.child_count = 1; // self

        entries.push(entry);
    }

    Ok(entries)
}

/// Recursively scans `root` on the **calling thread** and returns a list of
/// [`DirEntry`] values with bottom-up size aggregation.
///
/// Subdirectory sizes are accumulated into their parent before the parent
/// entry is appended, so the returned slice is in bottom-up (post-order) order.
///
/// Silently skips permission errors.  Respects `cancelled`.
pub fn scan_dir_sync(
    root: &Path,
    cancelled: &AtomicBool,
) -> Vec<DirEntry> {
    let mut results = Vec::new();
    scan_recursive(root, 0, cancelled, &mut results);
    results
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Recursive helper for [`scan_dir_sync`].
///
/// Returns the total size (bytes) of all files under `path` so the caller can
/// accumulate it into the parent entry.
fn scan_recursive(
    path: &Path,
    depth: usize,
    cancelled: &AtomicBool,
    results: &mut Vec<DirEntry>,
) -> u64 {
    if cancelled.load(Ordering::Relaxed) {
        return 0;
    }

    let read_dir = match std::fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return 0,
        Err(_) => return 0,
    };

    let mut entry = DirEntry::new(path.to_path_buf(), depth);

    for result in read_dir {
        if cancelled.load(Ordering::Relaxed) {
            break;
        }

        let dir_entry = match result {
            Ok(de) => de,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => continue,
            Err(_) => continue,
        };

        let child_path = dir_entry.path();
        entry.child_count += 1;

        let meta = match dir_entry.metadata() {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => continue,
            Err(_) => continue,
        };

        if meta.is_dir() {
            // Recurse first (post-order / bottom-up).
            let child_size = scan_recursive(&child_path, depth + 1, cancelled, results);
            entry.size += child_size;
        } else {
            entry.size += meta.len();
        }
    }

    let total_size = entry.size;
    results.push(entry);
    total_size
}

/// Returns a reasonable number of worker threads to use (capped at
/// [`MAX_WORKERS`]).  Falls back to 1 if the CPU count cannot be determined.
fn num_cpus() -> usize {
    // std::thread::available_parallelism was stabilised in Rust 1.59.
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(MAX_WORKERS)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    fn temp_tree() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();

        std::fs::write(root.join("a.txt"), b"hello").unwrap();
        std::fs::write(root.join("b.txt"), b"world!!").unwrap();
        std::fs::create_dir(root.join("sub")).unwrap();
        std::fs::write(root.join("sub").join("c.txt"), b"nested").unwrap();

        dir
    }

    #[test]
    fn read_dir_immediate_returns_children() {
        let dir = temp_tree();
        let entries = read_dir_immediate(dir.path()).expect("read_dir_immediate");
        assert_eq!(entries.len(), 3, "expected 3 children (a.txt, b.txt, sub)");
    }

    #[test]
    fn scan_dir_sync_aggregates_sizes() {
        let dir = temp_tree();
        let cancelled = AtomicBool::new(false);
        let results = scan_dir_sync(dir.path(), &cancelled);

        // Should have at least the root and the sub directory.
        assert!(results.len() >= 2);

        // The root entry (last in bottom-up order) should have the largest size.
        let root_entry = results.last().expect("at least one entry");
        assert!(root_entry.size > 0);
    }

    #[test]
    fn scan_dir_sync_respects_cancellation() {
        let dir = temp_tree();
        let cancelled = AtomicBool::new(true); // already cancelled
        let results = scan_dir_sync(dir.path(), &cancelled);
        assert!(results.is_empty(), "cancelled scan should return no results");
    }

    #[test]
    fn scan_dir_incremental_sends_complete() {
        let dir = temp_tree();
        let cancelled = Arc::new(AtomicBool::new(false));
        let (tx, rx) = std::sync::mpsc::channel::<ScanMessage>();

        scan_dir_incremental(dir.path().to_path_buf(), tx, cancelled);

        let messages: Vec<ScanMessage> = rx.try_iter().collect();
        let has_complete = messages
            .iter()
            .any(|m| matches!(m, ScanMessage::Complete));
        assert!(has_complete, "expected a Complete message");
    }

    #[test]
    fn scan_dir_incremental_cancellation() {
        let dir = temp_tree();
        let cancelled = Arc::new(AtomicBool::new(true)); // cancel immediately
        let (tx, rx) = std::sync::mpsc::channel::<ScanMessage>();

        scan_dir_incremental(dir.path().to_path_buf(), tx, cancelled);

        let messages: Vec<ScanMessage> = rx.try_iter().collect();
        let has_complete = messages
            .iter()
            .any(|m| matches!(m, ScanMessage::Complete));
        assert!(has_complete, "Complete must always be sent");
    }
}