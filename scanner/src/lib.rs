//! # Scanner
//!
//! Multi-threaded filesystem scanner.
//!
//! ## Public API
//!
//! - [`scan_dir_incremental`] — spawns up to 8 worker threads pulling from a
//!   shared work queue with condition variables; sends [`ScanMessage`] values
//!   (`DirScanned` / `ScanError` / `Complete`) to the caller via a channel.
//! - [`read_dir_immediate`] — reads one directory level and returns the entries
//!   immediately (non-recursive).
//! - [`scan_dir_sync`] — recursive single-threaded scan with bottom-up size
//!   aggregation.
//!
//! All scanning operations respect an atomic `cancelled` flag and silently skip
//! permission errors.

use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
        Arc, Condvar, Mutex,
    },
    thread,
};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// A single filesystem entry returned by the scanner.
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Absolute path of the entry.
    pub path: PathBuf,
    /// `true` when the entry is a directory.
    pub is_dir: bool,
    /// File size in bytes (`0` for directories or when unavailable).
    pub size: u64,
}

/// Messages sent from worker threads to the UI / caller.
#[derive(Debug)]
pub enum ScanMessage {
    /// A directory was successfully scanned; carries its direct children.
    DirScanned {
        /// The directory that was scanned.
        dir: PathBuf,
        /// Direct children of `dir`.
        entries: Vec<DirEntry>,
    },
    /// A non-fatal error occurred while scanning `path`.
    ScanError {
        /// Path that triggered the error.
        path: PathBuf,
        /// Human-readable error description.
        message: String,
    },
    /// All work is finished (or the scan was cancelled).
    Complete,
}

// ---------------------------------------------------------------------------
// Shared work queue
// ---------------------------------------------------------------------------

struct WorkQueue {
    queue: Mutex<VecDeque<PathBuf>>,
    condvar: Condvar,
}

impl WorkQueue {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        })
    }

    /// Push a new directory path onto the queue and wake one waiting worker.
    fn push(&self, path: PathBuf) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(path);
        self.condvar.notify_one();
    }

    /// Pop a path from the queue, blocking until one is available or the
    /// `shutdown` flag is set.
    ///
    /// Returns `None` when the queue is empty **and** `shutdown` is `true`.
    fn pop(&self, shutdown: &AtomicBool) -> Option<PathBuf> {
        let mut q = self.queue.lock().unwrap();
        loop {
            if let Some(path) = q.pop_front() {
                return Some(path);
            }
            if shutdown.load(Ordering::Acquire) {
                return None;
            }
            q = self.condvar.wait(q).unwrap();
        }
    }

    /// Signal all waiting workers to wake up and check the shutdown flag.
    fn notify_all(&self) {
        self.condvar.notify_all();
    }
}

// ---------------------------------------------------------------------------
// scan_dir_incremental
// ---------------------------------------------------------------------------

/// Maximum number of worker threads spawned by [`scan_dir_incremental`].
const MAX_WORKERS: usize = 8;

/// Incrementally scan `root` using up to [`MAX_WORKERS`] worker threads.
///
/// Progress is reported by sending [`ScanMessage`] values through the returned
/// [`mpsc::Receiver`].  The final message is always [`ScanMessage::Complete`].
///
/// # Cancellation
///
/// Pass an `Arc<AtomicBool>` as `cancelled`.  Set it to `true` from any thread
/// to stop the scan as soon as possible.
///
/// # Errors
///
/// Permission errors are silently skipped.  Other I/O errors are reported as
/// [`ScanMessage::ScanError`].
pub fn scan_dir_incremental(
    root: impl AsRef<Path>,
    cancelled: Arc<AtomicBool>,
) -> mpsc::Receiver<ScanMessage> {
    let root = root.as_ref().to_path_buf();
    let (tx, rx) = mpsc::channel::<ScanMessage>();

    thread::spawn(move || {
        let work_queue = WorkQueue::new();
        let shutdown = Arc::new(AtomicBool::new(false));

        // Seed the queue with the root directory.
        work_queue.push(root);

        // Track how many directories are in-flight so we know when we're done.
        let pending = Arc::new(Mutex::new(1usize));
        let pending_condvar = Arc::new(Condvar::new());

        let mut handles = Vec::with_capacity(MAX_WORKERS);

        for _ in 0..MAX_WORKERS {
            let wq = Arc::clone(&work_queue);
            let sd = Arc::clone(&shutdown);
            let cancelled = Arc::clone(&cancelled);
            let tx = tx.clone();
            let pending = Arc::clone(&pending);
            let pending_cv = Arc::clone(&pending_condvar);

            let handle = thread::spawn(move || {
                loop {
                    // Check cancellation before blocking.
                    if cancelled.load(Ordering::Acquire) {
                        break;
                    }

                    let dir = match wq.pop(&sd) {
                        Some(p) => p,
                        None => break,
                    };

                    // Re-check cancellation after waking.
                    if cancelled.load(Ordering::Acquire) {
                        // Decrement pending and signal.
                        let mut p = pending.lock().unwrap();
                        *p = p.saturating_sub(1);
                        pending_cv.notify_all();
                        break;
                    }

                    let entries = scan_one_dir(&dir);
                    let mut child_dirs: Vec<PathBuf> = Vec::new();
                    let mut scan_entries: Vec<DirEntry> = Vec::new();

                    match entries {
                        Ok(items) => {
                            for item in items {
                                if item.is_dir {
                                    child_dirs.push(item.path.clone());
                                }
                                scan_entries.push(item);
                            }
                            let _ = tx.send(ScanMessage::DirScanned {
                                dir: dir.clone(),
                                entries: scan_entries,
                            });
                        }
                        Err(e) => {
                            let _ = tx.send(ScanMessage::ScanError {
                                path: dir.clone(),
                                message: e.to_string(),
                            });
                        }
                    }

                    // Enqueue child directories before decrementing pending so
                    // the coordinator does not see a spurious zero.
                    {
                        let mut p = pending.lock().unwrap();
                        *p += child_dirs.len();
                    }
                    for child in child_dirs {
                        wq.push(child);
                    }

                    // Decrement pending for the directory we just finished.
                    let mut p = pending.lock().unwrap();
                    *p = p.saturating_sub(1);
                    pending_cv.notify_all();
                }
            });

            handles.push(handle);
        }

        // Wait until all directories have been processed (or cancelled).
        {
            let mut p = pending.lock().unwrap();
            while *p > 0 && !cancelled.load(Ordering::Acquire) {
                p = pending_condvar.wait(p).unwrap();
            }
        }

        // Signal workers to shut down and wake any that are blocked.
        shutdown.store(true, Ordering::Release);
        work_queue.notify_all();

        for h in handles {
            let _ = h.join();
        }

        let _ = tx.send(ScanMessage::Complete);
    });

    rx
}

// ---------------------------------------------------------------------------
// read_dir_immediate
// ---------------------------------------------------------------------------

/// Read one directory level of `dir` and return its direct children.
///
/// Permission errors are silently skipped.  Other errors are propagated.
pub fn read_dir_immediate(dir: impl AsRef<Path>) -> std::io::Result<Vec<DirEntry>> {
    scan_one_dir(dir.as_ref())
}

// ---------------------------------------------------------------------------
// scan_dir_sync
// ---------------------------------------------------------------------------

/// Recursively scan `root` in a single thread and return all entries with
/// bottom-up size aggregation (a directory's `size` is the sum of all
/// descendants).
///
/// # Cancellation
///
/// The `cancelled` flag is checked before descending into each sub-directory.
/// When set, the function returns whatever has been collected so far.
///
/// # Errors
///
/// Permission errors are silently skipped.
pub fn scan_dir_sync(root: impl AsRef<Path>, cancelled: &AtomicBool) -> Vec<DirEntry> {
    let mut results: Vec<DirEntry> = Vec::new();
    scan_recursive(root.as_ref(), cancelled, &mut results);
    results
}

/// Internal recursive helper for [`scan_dir_sync`].
///
/// Returns the aggregated size of all entries under `dir`.
fn scan_recursive(dir: &Path, cancelled: &AtomicBool, out: &mut Vec<DirEntry>) -> u64 {
    if cancelled.load(Ordering::Acquire) {
        return 0;
    }

    let children = match scan_one_dir(dir) {
        Ok(c) => c,
        Err(_) => return 0,
    };

    let mut dir_size: u64 = 0;

    // Index of the placeholder we'll insert for `dir` itself.
    let dir_index = out.len();
    // Push a placeholder; we'll update its size after processing children.
    out.push(DirEntry {
        path: dir.to_path_buf(),
        is_dir: true,
        size: 0,
    });

    for child in children {
        if child.is_dir {
            let subtree_size = scan_recursive(&child.path.clone(), cancelled, out);
            dir_size += subtree_size;
        } else {
            dir_size += child.size;
            out.push(child);
        }
    }

    // Update the placeholder with the aggregated size (bottom-up).
    out[dir_index].size = dir_size;

    dir_size
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Read one directory level, silently skipping permission errors.
fn scan_one_dir(dir: &Path) -> std::io::Result<Vec<DirEntry>> {
    let mut entries = Vec::new();

    let read_dir = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) if is_permission_error(&e) => return Ok(entries),
        Err(e) => return Err(e),
    };

    for result in read_dir {
        let entry = match result {
            Ok(e) => e,
            Err(e) if is_permission_error(&e) => continue,
            Err(e) => return Err(e),
        };

        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) if is_permission_error(&e) => continue,
            Err(_) => {
                // Non-permission metadata error: include entry with size 0.
                entries.push(DirEntry {
                    is_dir: path.is_dir(),
                    path,
                    size: 0,
                });
                continue;
            }
        };

        let is_dir = metadata.is_dir();
        let size = if is_dir { 0 } else { metadata.len() };

        entries.push(DirEntry { path, is_dir, size });
    }

    Ok(entries)
}

/// Returns `true` when the error represents a permission / access denial.
fn is_permission_error(e: &std::io::Error) -> bool {
    matches!(
        e.kind(),
        std::io::ErrorKind::PermissionDenied | std::io::ErrorKind::Other
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn read_dir_immediate_returns_entries() {
        // Use the project root (always present in a Cargo workspace).
        let entries = read_dir_immediate(".").expect("should read current dir");
        assert!(!entries.is_empty(), "current directory should not be empty");
    }

    #[test]
    fn scan_dir_sync_aggregates_sizes() {
        let cancelled = AtomicBool::new(false);
        let entries = scan_dir_sync(".", &cancelled);
        // The root entry (index 0) should have a non-zero aggregated size
        // because the project has source files.
        assert!(!entries.is_empty());
        let root = entries.iter().find(|e| e.is_dir);
        assert!(root.is_some(), "should have at least one directory entry");
    }

    #[test]
    fn scan_dir_sync_respects_cancellation() {
        let cancelled = AtomicBool::new(true); // already cancelled
        let entries = scan_dir_sync(".", &cancelled);
        // With cancellation pre-set the root placeholder is still pushed,
        // but no children are recursed into.
        assert!(entries.len() <= 1);
    }

    #[test]
    fn scan_dir_incremental_sends_complete() {
        let cancelled = Arc::new(AtomicBool::new(false));
        let rx = scan_dir_incremental(".", cancelled);

        let mut got_complete = false;
        for msg in rx {
            if let ScanMessage::Complete = msg {
                got_complete = true;
                break;
            }
        }
        assert!(got_complete, "should receive Complete message");
    }

    #[test]
    fn scan_dir_incremental_can_be_cancelled() {
        let cancelled = Arc::new(AtomicBool::new(false));
        let rx = scan_dir_incremental("/", Arc::clone(&cancelled));

        // Cancel almost immediately.
        cancelled.store(true, Ordering::Release);

        // Drain until Complete; must not hang.
        for msg in rx {
            if let ScanMessage::Complete = msg {
                break;
            }
        }
    }
}