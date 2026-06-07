//! Filesystem tree node representation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a node in the filesystem tree, capturing metadata about a file
/// or directory discovered during a scan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsNode {
    /// Display name of the file or directory (last path component).
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Current size in bytes (sum of all descendants for directories).
    pub size: u64,

    /// Size in bytes from the previous scan, used for delta calculations.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node (0 for files).
    pub file_count: u64,

    /// Number of folders contained within this node (0 for files).
    pub folder_count: u64,

    /// Whether this node represents a directory (`true`) or a file (`false`).
    pub is_dir: bool,

    /// Last modification timestamp of the file or directory.
    pub modified: Option<DateTime<Utc>>,

    /// Direct children of this node (populated for directories).
    pub children: Vec<FsNode>,
}

impl FsNode {
    /// Creates a new [`FsNode`] for a directory with the given name and path.
    pub fn new_dir(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            size: 0,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            is_dir: true,
            modified: None,
            children: Vec::new(),
        }
    }

    /// Creates a new [`FsNode`] for a file with the given name, path, and size.
    pub fn new_file(name: impl Into<String>, path: impl Into<String>, size: u64) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            size,
            prev_size: None,
            file_count: 1,
            folder_count: 0,
            is_dir: false,
            modified: None,
            children: Vec::new(),
        }
    }

    /// Returns `true` if this node represents a directory.
    pub fn is_directory(&self) -> bool {
        self.is_dir
    }

    /// Returns the total number of descendant entries (files + folders).
    pub fn total_entry_count(&self) -> u64 {
        self.file_count + self.folder_count
    }

    /// Returns the size delta compared to the previous scan, if available.
    pub fn size_delta(&self) -> Option<i64> {
        self.prev_size
            .map(|prev| self.size as i64 - prev as i64)
    }
}

impl Default for FsNode {
    fn default() -> Self {
        Self::new_dir(String::new(), String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dir_sets_is_dir_true() {
        let node = FsNode::new_dir("docs", "/home/user/docs");
        assert!(node.is_dir);
        assert_eq!(node.name, "docs");
        assert_eq!(node.path, "/home/user/docs");
        assert_eq!(node.size, 0);
        assert!(node.children.is_empty());
    }

    #[test]
    fn new_file_sets_is_dir_false() {
        let node = FsNode::new_file("readme.txt", "/home/user/readme.txt", 1024);
        assert!(!node.is_dir);
        assert_eq!(node.size, 1024);
        assert_eq!(node.file_count, 1);
    }

    #[test]
    fn size_delta_returns_none_without_prev_size() {
        let node = FsNode::new_file("a.txt", "/a.txt", 500);
        assert_eq!(node.size_delta(), None);
    }

    #[test]
    fn size_delta_returns_correct_delta() {
        let mut node = FsNode::new_file("a.txt", "/a.txt", 500);
        node.prev_size = Some(300);
        assert_eq!(node.size_delta(), Some(200));
    }
}