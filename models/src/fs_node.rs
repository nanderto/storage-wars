//! Filesystem tree node representation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a node in the filesystem tree, capturing both current and
/// previous scan data to enable size-change comparisons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsNode {
    /// Display name of the file or directory (last path component).
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Current total size in bytes (includes children for directories).
    pub size: u64,

    /// Size in bytes from the previous scan, if available.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node (0 for files).
    pub file_count: u64,

    /// Number of folders contained within this node (0 for files).
    pub folder_count: u64,

    /// Whether this node represents a directory.
    pub is_dir: bool,

    /// Last modification timestamp of the file or directory.
    pub modified: Option<DateTime<Utc>>,

    /// Child nodes (populated for directories).
    pub children: Vec<FsNode>,
}

impl FsNode {
    /// Creates a new `FsNode` for a file.
    pub fn new_file(name: impl Into<String>, path: impl Into<String>, size: u64) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            size,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            is_dir: false,
            modified: None,
            children: Vec::new(),
        }
    }

    /// Creates a new `FsNode` for a directory.
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

    /// Returns the size delta between the current and previous scan.
    /// Returns `None` if no previous size is recorded.
    pub fn size_delta(&self) -> Option<i64> {
        self.prev_size
            .map(|prev| self.size as i64 - prev as i64)
    }

    /// Returns `true` if this node has no children.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns the total number of descendant nodes (files + folders).
    pub fn total_count(&self) -> u64 {
        self.file_count + self.folder_count
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
    fn test_new_file() {
        let node = FsNode::new_file("file.txt", "/home/user/file.txt", 1024);
        assert_eq!(node.name, "file.txt");
        assert_eq!(node.path, "/home/user/file.txt");
        assert_eq!(node.size, 1024);
        assert!(!node.is_dir);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_new_dir() {
        let node = FsNode::new_dir("documents", "/home/user/documents");
        assert_eq!(node.name, "documents");
        assert!(node.is_dir);
        assert_eq!(node.size, 0);
    }

    #[test]
    fn test_size_delta_with_prev() {
        let mut node = FsNode::new_file("file.txt", "/file.txt", 2048);
        node.prev_size = Some(1024);
        assert_eq!(node.size_delta(), Some(1024));
    }

    #[test]
    fn test_size_delta_no_prev() {
        let node = FsNode::new_file("file.txt", "/file.txt", 2048);
        assert_eq!(node.size_delta(), None);
    }

    #[test]
    fn test_total_count() {
        let mut node = FsNode::new_dir("root", "/root");
        node.file_count = 10;
        node.folder_count = 3;
        assert_eq!(node.total_count(), 13);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let node = FsNode::new_file("test.rs", "/src/test.rs", 512);
        let json = serde_json::to_string(&node).expect("serialization failed");
        let restored: FsNode = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(node, restored);
    }
}