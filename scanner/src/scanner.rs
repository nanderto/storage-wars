//! Core scanning logic: incremental, immediate, and synchronous strategies.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};

use crate::error::ScanError;
use crate::message::ScanMessage;
use crate::worker::{WorkQueue, Worker};

// ---------------------------------------------------------------------------
// Public data structures
// ---------------------------------------------------------------------------

/// Represents a scanned directory with aggregated size information.
#[derive(Debug, Clone)]
pub struct DirNode {
    /// Absolute path of this directory.
    pub path: PathBuf,
    /// Total size in bytes of all files reachable from this directory.
    pub size: u64,
    /// Immediate children (files and sub-directories) discovered in this node.
    pub children: Vec<FsEntry>,
}

/// A single filesystem entry (file or directory) discovered during scanning.
#[derive(Debug, Clone)]
pub struct FsEntry {
    /// Path of this entry.
    pub path: PathBuf,
    /// `true` if this entry is a directory.
    pub is_dir: bool,
    /// Size in bytes. For directories this reflects the aggregated subtree size
    /// only when produced by [`scan_dir_sync`]; otherwise it is the raw
    /// `metadata().len()` value.
    pub size: u64,
}

// ---------------------------------------------------------------------------
// read_dir_immediate
// ---------------------------------------------------------------------------

/// Reads a single directory level and returns all entries.
///
/// Permission errors are silently skipped. Other I/O errors are returned.
///
/// # Errors
///
/// Returns [`ScanError::Io`] if the directory itself cannot be opened.
pub fn read_dir_immediate(path: &Path) -> Result<Vec<FsEntry>, ScanError> {
    let read_dir = fs::read_dir(path).map_err(|e| ScanError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    let mut entries = Vec::new();

    for result in read_dir {
        let dir_entry = match result {
            Ok(e) => e,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
            Err(e) => {
                return Err(ScanError::Io {
                    path: path.to_path_buf(),
                    source: e,
                })
            }
        };

        let entry_path = dir_entry.path();
        let metadata = match dir_entry.metadata() {
            Ok(m) => m,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
            Err(e) => {
                return Err(ScanError::Io {
                    path: entry_path,
                    source: e,
                })
            }
        };

        entries.push(FsEntry {
            path: entry_path,
            is_dir: metadata.is_dir(),
            size: metadata.len(),
        });
    }

    Ok(entries)
}

// ---------------------------------------------------------------------------
// scan_dir_sync
// ---------------------------------------------------------------------------

/// Recursively scans `root` in a single thread, aggregating sizes bottom-up.
///
/// Returns a [`DirNode`] whose `size` field reflects the total size of all
/// files in the entire subtree.
///
/// Respects the `cancelled` flag; returns [`ScanError::Cancelled`] if set.
/// Permission errors are silently skipped.
///
/// # Errors
///
/// Returns [`ScanError::Io`] on unexpected I/O failures, or
/// [`ScanError::Cancelled`] if the atomic flag is set.
pub fn scan_dir_sync(
    root: &Path,
    cancelled: Arc<AtomicBool>,
) -> Result<DirNode, ScanError> {
    scan_recursive(root, &cancelled)
}

fn scan_recursive(path: &Path, cancelled: &Arc<AtomicBool>) -> Result<DirNode, ScanError> {
    if cancelled.load(Ordering::Relaxed) {
        return Err(ScanError::Cancelled);
    }

    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            // Return an empty node for inaccessible directories.
            return Ok(DirNode {
                path: path.to_path_buf(),
                size: 0,
                children: Vec::new(),
            });
        }
        Err(e) => {
            return Err(ScanError::Io {
                path: path.to_path_buf(),
                source: e,
            })
        }
    };

    let mut children: Vec<FsEntry> = Vec::new();
    let mut total_size: u64 = 0;

    for result in read_dir {
        if cancelled.load(Ordering::Relaxed) {
            return Err(ScanError::Cancelled);
        }

        let dir_entry = match result {
            Ok(e) => e,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
            Err(e) => {
                return Err(ScanError::Io {
                    path: path.to_path_buf(),
                    source: e,
                })
            }
        };

        let entry_path = dir_entry.path();
        let metadata = match dir_entry.metadata() {
            Ok(m) => m,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
            Err(_) => continue,
        };

        if metadata.is_dir() {
            // Recurse — bottom-up aggregation.
            match scan_recursive(&entry_path, cancelled) {
                Ok(child_node) => {
                    let child_size = child_node.size;
                    total_size += child_size;
                    children.push(FsEntry {
                        path: entry_path,
                        is_dir: true,
                        size: child_size,
                    });
                    // child_node is consumed; callers receive the aggregated root.
                }
                Err(ScanError::Cancelled) => return Err(ScanError::Cancelled),
                Err(_) => continue, // skip unreadable sub-trees
            }
        } else {
            let file_size = metadata.len();
            total_size += file_size;
            children.push(FsEntry {
                path: entry_path,
                is_dir: false,
                size: file_size,
            });
        }
    }

    Ok(DirNode {
        path: path.to_path_buf(),
        size: total_size,
        children,
    })
}

// ---------------------------------------------------------------------------
// scan_dir_incremental
// ---------------------------------------------------------------------------

/// Maximum number of worker threads spawned by [`scan_dir_incremental`].
const MAX_WORKERS: usize = 8;

/// Incrementally scans `root` using up to [`MAX_WORKERS`] threads.
///
/// Each scanned directory produces a [`ScanMessage::DirScanned`] message on
/// `tx`. Non-fatal errors produce [`ScanMessage::ScanError`]. When the scan
/// finishes (or is cancelled) a single [`ScanMessage::Complete`] is sent.
///
/// Permission errors are silently skipped and never produce a `ScanError`
/// message.
///
/// The `cancelled` flag can be set from any thread to abort the scan
/// cooperatively.
pub fn scan_dir_incremental(
    root: PathBuf,
    tx: Sender<ScanMessage>,
    cancelled: Arc<AtomicBool>,
) {
    // Shared work queue: directories yet to be scanned.
    let queue = Arc::new(WorkQueue::new());
    queue.push(root);

    // Spawn worker threads.
    let mut handles = Vec::with_capacity(MAX_WORKERS);

    for _ in 0..MAX_WORKERS {
        let worker = Worker::new(
            Arc::clone(&queue),
            tx.clone(),
            Arc::clone(&cancelled),
        );
        handles.push(std::thread::spawn(move || worker.run()));
    }

    // Wait for all workers to finish.
    for handle in handles {
        // Ignore panics in individual workers — the Complete message is sent
        // by the last worker to exit via the WorkQueue sentinel.
        let _ = handle.join();
    }

    // Send the final Complete message.  If the channel is already closed
    // (receiver dropped) we simply ignore the error.
    let _ = tx.send(ScanMessage::Complete);
}