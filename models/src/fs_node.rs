//! Filesystem tree node representation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a node in the filesystem tree, which can be either a file or a
/// directory. Stores the current size, previous size (for delta comparison),
/// child counts, and the last-modified timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsNode {
    /// The name of the file or directory (not the full path).
    pub name: String,

    /// The absolute path of the file or directory.
    pub path: String,

    /// The current size in bytes.
    pub size: u64,

    /// The size in bytes from the previous scan, if available.
    pub prev_size: Option<u64>,

    /// The number of files contained within this node (0 for files).
    pub file_count: u64,

    /// The number of folders contained within this node (0 for files).
    pub folder_count: u64,

    /// Whether this node represents a file (`true`) or a directory (`false`).
    pub is_file: bool,

    /// The last-modified timestamp of the file or directory.
    pub modified: Option<DateTime<Utc>>,

    /// Child nodes, populated for directories.
    pub children: Vec<FsNode>,
}

impl FsNode {
    /// Creates a new [`FsNode`] representing a file.
    pub fn new_file(
        name: impl Into<String>,
        path: impl Into<String>,
        size: u64,
        modified: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            size,
            prev_size: None,
            file_count: 1,
            folder_count: 0,
            is_file: true,
            modified,
            children: Vec::new(),
        }
    }

    /// Creates a new [`FsNode`] representing a directory.
    pub fn new_dir(
        name: impl Into<String>,
        path: impl Into<String>,
        modified: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            size: 0,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            is_file: false,
            modified,
            children: Vec::new(),
        }
    }

    /// Returns the size delta between the current and previous scan.
    /// Returns `None` if no previous size is recorded.
    pub fn size_delta(&self) -> Option<i64> {
        self.prev_size
            .map(|prev| self.size as i64 - prev as i64)
    }

    /// Returns `true` if this node represents a directory.
    pub fn is_dir(&self) -> bool {
        !self.is_file
    }
}

impl Default for FsNode {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
            size: 0,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            is_file: false,
            modified: None,
            children: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_file_sets_is_file_true() {
        let node = FsNode::new_file("foo.txt", "/tmp/foo.txt", 1024, None);
        assert!(node.is_file);
        assert_eq!(node.file_count, 1);
        assert_eq!(node.folder_count, 0);
    }

    #[test]
    fn new_dir_sets_is_file_false() {
        let node = FsNode::new_dir("docs", "/tmp/docs", None);
        assert!(!node.is_file);
        assert!(node.is_dir());
    }

    #[test]
    fn size_delta_none_when_no_prev_size() {
        let node = FsNode::new_file("a.txt", "/a.txt", 500, None);
        assert_eq!(node.size_delta(), None);
    }

    #[test]
    fn size_delta_computed_correctly() {
        let mut node = FsNode::new_file("a.txt", "/a.txt", 500, None);
        node.prev_size = Some(300);
        assert_eq!(node.size_delta(), Some(200));
    }
}