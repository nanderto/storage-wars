//! Filesystem tree node representation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a node in the filesystem tree, which can be either a file or a folder.
///
/// Stores the current and previous size for delta tracking, along with
/// file/folder counts and the last modified timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsNode {
    /// Display name of the file or directory.
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Current size in bytes.
    pub size: u64,

    /// Previous size in bytes, used for change detection. `None` if no prior scan exists.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node (0 for files, recursive count for folders).
    pub file_count: u64,

    /// Number of folders contained within this node (0 for files, recursive count for folders).
    pub folder_count: u64,

    /// Last modified timestamp of the file or directory.
    pub modified: Option<DateTime<Utc>>,

    /// Whether this node represents a directory (`true`) or a file (`false`).
    pub is_dir: bool,

    /// Child nodes, populated only when `is_dir` is `true`.
    pub children: Vec<FsNode>,
}

impl FsNode {
    /// Creates a new `FsNode` representing a file.
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
            modified,
            is_dir: false,
            children: Vec::new(),
        }
    }

    /// Creates a new `FsNode` representing a directory.
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
            folder_count: 1,
            modified,
            is_dir: true,
            children: Vec::new(),
        }
    }

    /// Returns the size delta between the current and previous size.
    ///
    /// Returns `None` if no previous size is recorded.
    pub fn size_delta(&self) -> Option<i64> {
        self.prev_size
            .map(|prev| self.size as i64 - prev as i64)
    }

    /// Returns `true` if this node has no children (applies to directories).
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Adds a child node to this directory node.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if called on a file node.
    pub fn add_child(&mut self, child: FsNode) {
        debug_assert!(self.is_dir, "Cannot add children to a file node");
        self.children.push(child);
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
            modified: None,
            is_dir: false,
            children: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_file() {
        let node = FsNode::new_file("file.txt", "/home/user/file.txt", 1024, None);
        assert_eq!(node.name, "file.txt");
        assert_eq!(node.path, "/home/user/file.txt");
        assert_eq!(node.size, 1024);
        assert!(!node.is_dir);
        assert_eq!(node.file_count, 1);
        assert_eq!(node.folder_count, 0);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_new_dir() {
        let node = FsNode::new_dir("docs", "/home/user/docs", None);
        assert_eq!(node.name, "docs");
        assert!(node.is_dir);
        assert_eq!(node.folder_count, 1);
    }

    #[test]
    fn test_size_delta_none_when_no_prev() {
        let node = FsNode::new_file("file.txt", "/file.txt", 500, None);
        assert_eq!(node.size_delta(), None);
    }

    #[test]
    fn test_size_delta_positive() {
        let mut node = FsNode::new_file("file.txt", "/file.txt", 1000, None);
        node.prev_size = Some(600);
        assert_eq!(node.size_delta(), Some(400));
    }

    #[test]
    fn test_size_delta_negative() {
        let mut node = FsNode::new_file("file.txt", "/file.txt", 400, None);
        node.prev_size = Some(600);
        assert_eq!(node.size_delta(), Some(-200));
    }

    #[test]
    fn test_add_child() {
        let mut dir = FsNode::new_dir("parent", "/parent", None);
        let child = FsNode::new_file("child.txt", "/parent/child.txt", 256, None);
        dir.add_child(child);
        assert_eq!(dir.children.len(), 1);
    }

    #[test]
    fn test_default() {
        let node = FsNode::default();
        assert_eq!(node.size, 0);
        assert!(!node.is_dir);
        assert!(node.prev_size.is_none());
    }
}