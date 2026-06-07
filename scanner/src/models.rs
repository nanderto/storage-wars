//! Data models used throughout the scanner.

use std::path::PathBuf;

/// Represents a scanned directory with its aggregated size.
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Absolute path to the directory.
    pub path: PathBuf,

    /// Total size in bytes of all files contained recursively within this directory.
    pub size: u64,

    /// Number of direct children (files + subdirectories) in this directory.
    pub child_count: usize,

    /// Depth relative to the scan root (root itself is depth 0).
    pub depth: usize,
}

impl DirEntry {
    /// Creates a new [`DirEntry`] with the given path and depth.
    pub fn new(path: PathBuf, depth: usize) -> Self {
        Self {
            path,
            size: 0,
            child_count: 0,
            depth,
        }
    }
}