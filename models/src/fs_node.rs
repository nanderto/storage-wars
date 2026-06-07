//! Filesystem tree node representation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a node in the filesystem tree, capturing metadata for a file
/// or directory discovered during a scan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsNode {
    /// Display name of the file or directory (last path component).
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Current size in bytes.
    pub size: u64,

    /// Size in bytes from the previous scan session, if available.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node (0 for files).
    pub file_count: u64,

    /// Number of sub-folders contained within this node (0 for files).
    pub folder_count: u64,

    /// Whether this node represents a directory (`true`) or a file (`false`).
    pub is_dir: bool,

    /// Last modification timestamp of the file or directory.
    pub modified: Option<DateTime<Utc>>,
}

impl FsNode {
    /// Creates a new [`FsNode`] with the given name and path.
    ///
    /// All numeric fields default to zero, `is_dir` defaults to `false`,
    /// and optional fields default to `None`.
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            size: 0,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            is_dir: false,
            modified: None,
        }
    }

    /// Returns the size delta compared to the previous scan, if available.
    ///
    /// A positive value means the node grew; a negative value means it shrank.
    pub fn size_delta(&self) -> Option<i64> {
        self.prev_size
            .map(|prev| self.size as i64 - prev as i64)
    }

    /// Returns `true` if this node represents a directory.
    pub fn is_directory(&self) -> bool {
        self.is_dir
    }

    /// Returns the total number of children (files + folders).
    pub fn total_children(&self) -> u64 {
        self.file_count + self.folder_count
    }
}

impl Default for FsNode {
    fn default() -> Self {
        Self::new(String::new(), String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fs_node_defaults() {
        let node = FsNode::new("documents", "/home/user/documents");
        assert_eq!(node.name, "documents");
        assert_eq!(node.path, "/home/user/documents");
        assert_eq!(node.size, 0);
        assert!(node.prev_size.is_none());
        assert_eq!(node.file_count, 0);
        assert_eq!(node.folder_count, 0);
        assert!(!node.is_dir);
        assert!(node.modified.is_none());
    }

    #[test]
    fn test_size_delta_with_prev_size() {
        let mut node = FsNode::new("file.txt", "/tmp/file.txt");
        node.size = 2000;
        node.prev_size = Some(1000);
        assert_eq!(node.size_delta(), Some(1000));
    }

    #[test]
    fn test_size_delta_shrunk() {
        let mut node = FsNode::new("file.txt", "/tmp/file.txt");
        node.size = 500;
        node.prev_size = Some(1000);
        assert_eq!(node.size_delta(), Some(-500));
    }

    #[test]
    fn test_size_delta_no_prev() {
        let node = FsNode::new("file.txt", "/tmp/file.txt");
        assert!(node.size_delta().is_none());
    }

    #[test]
    fn test_total_children() {
        let mut node = FsNode::new("dir", "/tmp/dir");
        node.is_dir = true;
        node.file_count = 10;
        node.folder_count = 3;
        assert_eq!(node.total_children(), 13);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut node = FsNode::new("test", "/tmp/test");
        node.size = 4096;
        node.is_dir = true;
        node.file_count = 5;

        let json = serde_json::to_string(&node).expect("serialization failed");
        let restored: FsNode = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(node, restored);
    }
}