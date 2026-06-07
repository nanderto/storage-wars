//! Filesystem tree node representation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a node in the filesystem tree, which can be either a file or a
/// directory. Stores the current size alongside the previous size to allow
/// delta calculations between scan sessions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsNode {
    /// Display name of the file or directory (last path component).
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Current size in bytes (sum of all children for directories).
    pub size: u64,

    /// Size in bytes recorded during the previous scan session, if any.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node (0 for files, recursive
    /// count for directories).
    pub file_count: u64,

    /// Number of sub-directories contained within this node (0 for files,
    /// recursive count for directories).
    pub folder_count: u64,

    /// Last modification timestamp of the file or directory.
    pub modified: Option<DateTime<Utc>>,

    /// Whether this node represents a directory (`true`) or a file (`false`).
    pub is_dir: bool,

    /// Child nodes (populated for directories; empty for files).
    pub children: Vec<FsNode>,
}

impl FsNode {
    /// Creates a new [`FsNode`] with the given name and path, defaulting all
    /// numeric fields to zero and leaving optional fields as `None`.
    pub fn new(name: impl Into<String>, path: impl Into<String>, is_dir: bool) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            size: 0,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            modified: None,
            is_dir,
            children: Vec::new(),
        }
    }

    /// Returns the size delta between the current and previous scan, if a
    /// previous size is available.
    pub fn size_delta(&self) -> Option<i64> {
        self.prev_size
            .map(|prev| self.size as i64 - prev as i64)
    }
}

impl Default for FsNode {
    fn default() -> Self {
        Self::new(String::new(), String::new(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fs_node_has_zero_counts() {
        let node = FsNode::new("docs", "/home/user/docs", true);
        assert_eq!(node.size, 0);
        assert_eq!(node.file_count, 0);
        assert_eq!(node.folder_count, 0);
        assert!(node.children.is_empty());
    }

    #[test]
    fn size_delta_returns_none_without_prev_size() {
        let node = FsNode::new("file.txt", "/home/user/file.txt", false);
        assert_eq!(node.size_delta(), None);
    }

    #[test]
    fn size_delta_returns_correct_delta() {
        let mut node = FsNode::new("file.txt", "/home/user/file.txt", false);
        node.size = 200;
        node.prev_size = Some(100);
        assert_eq!(node.size_delta(), Some(100));
    }
}