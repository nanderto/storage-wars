//! Worker thread implementation for the incremental scanner.

use crate::messages::ScanMessage;
use crate::models::DirEntry;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};

/// Shared work queue state protected by a mutex.
pub struct WorkQueue {
    /// Pending directories to scan, each paired with their depth.
    pub items: Vec<(PathBuf, usize)>,
    /// Number of worker threads currently processing an item.
    pub in_flight: usize,
    /// Set to true once no more work will be added.
    pub done: bool,
}

impl WorkQueue {
    pub fn new(root: PathBuf) -> Self {
        Self {
            items: vec![(root, 0)],
            in_flight: 0,
            done: false,
        }
    }
}

/// Shared handle passed to every worker thread.
pub type SharedQueue = Arc<(Mutex<WorkQueue>, Condvar)>;

/// Spawns worker threads that drain the shared work queue.
///
/// Each worker:
/// 1. Waits on the condvar until work is available or scanning is done.
/// 2. Pops a directory from the queue.
/// 3. Reads its entries, accumulates sizes, and pushes subdirectories back.
/// 4. Sends a [`ScanMessage::DirScanned`] for the processed directory.
pub fn run_worker(
    queue: SharedQueue,
    tx: std::sync::mpsc::Sender<ScanMessage>,
    cancelled: Arc<AtomicBool>,
) {
    let (lock, cvar) = &*queue;

    loop {
        // ── Wait for work ────────────────────────────────────────────────────
        let (path, depth) = {
            let mut guard = lock.lock().expect("work queue mutex poisoned");

            loop {
                if cancelled.load(Ordering::Relaxed) {
                    return;
                }

                if let Some(item) = guard.items.pop() {
                    guard.in_flight += 1;
                    break item;
                }

                // No items: check if we are truly done.
                if guard.done || guard.in_flight == 0 {
                    return;
                }

                // Park until notified.
                guard = cvar.wait(guard).expect("condvar wait failed");
            }
        };

        // ── Process the directory ────────────────────────────────────────────
        let mut entry = DirEntry::new(path.clone(), depth);
        let mut subdirs: Vec<(PathBuf, usize)> = Vec::new();

        match std::fs::read_dir(&path) {
            Err(e) => {
                // Silently skip permission errors; report everything else.
                if e.kind() != std::io::ErrorKind::PermissionDenied {
                    let _ = tx.send(ScanMessage::ScanError {
                        path: path.clone(),
                        error: e.to_string(),
                    });
                }
            }
            Ok(read_dir) => {
                for result in read_dir {
                    if cancelled.load(Ordering::Relaxed) {
                        break;
                    }

                    let dir_entry = match result {
                        Ok(de) => de,
                        Err(e) => {
                            if e.kind() != std::io::ErrorKind::PermissionDenied {
                                let _ = tx.send(ScanMessage::ScanError {
                                    path: path.clone(),
                                    error: e.to_string(),
                                });
                            }
                            continue;
                        }
                    };

                    let child_path = dir_entry.path();
                    entry.child_count += 1;

                    match dir_entry.metadata() {
                        Ok(meta) => {
                            if meta.is_dir() {
                                subdirs.push((child_path, depth + 1));
                            } else {
                                entry.size += meta.len();
                            }
                        }
                        Err(e) => {
                            if e.kind() != std::io::ErrorKind::PermissionDenied {
                                let _ = tx.send(ScanMessage::ScanError {
                                    path: child_path,
                                    error: e.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        // ── Push subdirectories back into the queue ──────────────────────────
        {
            let mut guard = lock.lock().expect("work queue mutex poisoned");
            guard.in_flight -= 1;
            guard.items.extend(subdirs);

            // If queue is now empty and nothing is in-flight, signal completion.
            if guard.items.is_empty() && guard.in_flight == 0 {
                guard.done = true;
            }

            cvar.notify_all();
        }

        // Send the scanned entry to the UI.
        let _ = tx.send(ScanMessage::DirScanned(entry));
    }
}