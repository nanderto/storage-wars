//! Worker thread implementation for incremental scanning.
//!
//! [`WorkQueue`] is a thread-safe queue of directories to scan, backed by a
//! `Mutex<VecDeque>` and a `Condvar` for efficient blocking.  [`Worker`]
//! pulls items from the queue and processes them, sending [`ScanMessage`]s
//! to the UI channel.

use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Condvar, Mutex};

use crate::message::ScanMessage;
use crate::scanner::{DirNode, FsEntry};

// ---------------------------------------------------------------------------
// WorkQueue
// ---------------------------------------------------------------------------

/// Internal state protected by the mutex.
struct QueueState {
    /// Pending directories to scan.
    pending: VecDeque<PathBuf>,
    /// Number of workers currently processing an item (not idle).
    active: usize,
    /// Set to `true` once the scan is considered done (queue empty + no active workers).
    done: bool,
}

/// Thread-safe work queue used by [`Worker`] threads.
pub struct WorkQueue {
    state: Mutex<QueueState>,
    condvar: Condvar,
}

impl WorkQueue {
    /// Creates a new, empty work queue.
    pub fn new() -> Self {
        WorkQueue {
            state: Mutex::new(QueueState {
                pending: VecDeque::new(),
                active: 0,
                done: false,
            }),
            condvar: Condvar::new(),
        }
    }

    /// Pushes a new directory path onto the queue and notifies one waiting worker.
    pub fn push(&self, path: PathBuf) {
        let mut state = self.state.lock().expect("WorkQueue mutex poisoned");
        state.pending.push_back(path);
        self.condvar.notify_one();
    }

    /// Pushes multiple directory paths and notifies all waiting workers.
    pub fn push_many(&self, paths: impl IntoIterator<Item = PathBuf>) {
        let mut state = self.state.lock().expect("WorkQueue mutex poisoned");
        let before = state.pending.len();
        state.pending.extend(paths);
        let added = state.pending.len() - before;
        for _ in 0..added {
            self.condvar.notify_one();
        }
    }

    /// Blocks until a work item is available or the queue is exhausted.
    ///
    /// Returns `Some(path)` when work is available, or `None` when the scan
    /// is complete (queue empty and no active workers).
    pub fn pop(&self) -> Option<PathBuf> {
        let mut state = self.state.lock().expect("WorkQueue mutex poisoned");

        loop {
            if state.done {
                // Wake any other waiters so they also exit.
                self.condvar.notify_all();
                return None;
            }

            if let Some(path) = state.pending.pop_front() {
                state.active += 1;
                return Some(path);
            }

            // Queue is empty — check if all workers are idle.
            if state.active == 0 {
                state.done = true;
                self.condvar.notify_all();
                return None;
            }

            // Some workers are still active; wait for them to push more work.
            state = self
                .condvar
                .wait(state)
                .expect("WorkQueue condvar wait failed");
        }
    }

    /// Called by a worker after it finishes processing an item.
    ///
    /// Decrements the active counter and notifies waiters so they can re-check
    /// the done condition.
    pub fn complete_item(&self) {
        let mut state = self.state.lock().expect("WorkQueue mutex poisoned");
        debug_assert!(state.active > 0, "complete_item called with active == 0");
        state.active -= 1;
        self.condvar.notify_all();
    }
}

// ---------------------------------------------------------------------------
// Worker
// ---------------------------------------------------------------------------

/// A single scanner worker that pulls directories from a [`WorkQueue`] and
/// sends [`ScanMessage`]s to the UI channel.
pub struct Worker {
    queue: Arc<WorkQueue>,
    tx: Sender<ScanMessage>,
    cancelled: Arc<AtomicBool>,
}

impl Worker {
    /// Creates a new worker.
    pub fn new(
        queue: Arc<WorkQueue>,
        tx: Sender<ScanMessage>,
        cancelled: Arc<AtomicBool>,
    ) -> Self {
        Worker { queue, tx, cancelled }
    }

    /// Runs the worker loop until the queue is exhausted or the scan is cancelled.
    pub fn run(self) {
        while let Some(dir_path) = self.queue.pop() {
            if self.cancelled.load(Ordering::Relaxed) {
                self.queue.complete_item();
                break;
            }

            self.process_directory(dir_path);
            self.queue.complete_item();
        }
    }

    /// Scans a single directory, enqueues sub-directories, and sends a message.
    fn process_directory(&self, dir_path: PathBuf) {
        let read_dir = match fs::read_dir(&dir_path) {
            Ok(rd) => rd,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                // Silently skip permission errors.
                return;
            }
            Err(e) => {
                let _ = self.tx.send(ScanMessage::ScanError {
                    path: dir_path,
                    error: e.to_string(),
                });
                return;
            }
        };

        let mut children: Vec<FsEntry> = Vec::new();
        let mut total_size: u64 = 0;
        let mut sub_dirs: Vec<PathBuf> = Vec::new();

        for result in read_dir {
            if self.cancelled.load(Ordering::Relaxed) {
                return;
            }

            let dir_entry = match result {
                Ok(e) => e,
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
                Err(e) => {
                    let _ = self.tx.send(ScanMessage::ScanError {
                        path: dir_path.clone(),
                        error: e.to_string(),
                    });
                    continue;
                }
            };

            let entry_path = dir_entry.path();
            let metadata = match dir_entry.metadata() {
                Ok(m) => m,
                Err(e) if e.kind() == io::ErrorKind::PermissionDenied => continue,
                Err(_) => continue,
            };

            let is_dir = metadata.is_dir();
            let size = if is_dir { 0 } else { metadata.len() };

            if is_dir {
                sub_dirs.push(entry_path.clone());
            } else {
                total_size += size;
            }

            children.push(FsEntry {
                path: entry_path,
                is_dir,
                size,
            });
        }

        // Enqueue discovered sub-directories for other workers to pick up.
        if !sub_dirs.is_empty() {
            self.queue.push_many(sub_dirs);
        }

        let node = DirNode {
            path: dir_path,
            size: total_size,
            children,
        };

        // Ignore send errors — the receiver may have been dropped (e.g. on cancel).
        let _ = self.tx.send(ScanMessage::DirScanned(node));
    }
}