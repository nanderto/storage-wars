//! Filesystem tree node representation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a node in the filesystem tree, which can be either a file or a directory.
///
/// Each node captures the current size, the previous size (for delta computation),
/// child file and folder counts, and the last-modified timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FsNode {
    /// The name of the file or directory (not the full path).
    pub name: String,

    /// The absolute path to this node on the filesystem.
    pub path: String,

    /// The total size in bytes of this node (including all descendants for directories).
    pub size: u64,

    /// The size in bytes from the previous scan, used to compute deltas.
    /// `None` if this node did not exist in the previous scan.
    pub prev_size: Option<u64>,

    /// The total number of files contained within this node (recursive).
    /// For a file node this is always `0`.
    pub file_count: u64,

    /// The total number of folders contained within this node (recursive).
    /// For a file node this is always `0`.
    pub folder_count: u64,

    /// Whether this node represents a directory (`true`) or a file (`false`).
    pub is_dir: bool,

    /// The last-modified timestamp reported by the filesystem.
    pub modified: Option<DateTime<Utc>>,

    /// Direct children of this node (empty for file nodes).
    #[serde(default)]
    pub children: Vec<FsNode>,
}

impl FsNode {
    /// Creates a new [`FsNode`] for a file.
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

    /// Creates a new [`FsNode`] for a directory.
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
    ///
    /// Returns `None` if there is no previous size to compare against.
    pub fn size_delta(&self) -> Option<i64> {
        self.prev_size
            .map(|prev| self.size as i64 - prev as i64)
    }

    /// Returns `true` if this node is a file.
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }

    /// Adds a child node and updates the aggregate counts and size.
    pub fn add_child(&mut self, child: FsNode) {
        if child.is_dir {
            self.folder_count += 1 + child.folder_count;
        } else {
            self.file_count += 1;
        }
        self.file_count += child.file_count;
        self.size += child.size;
        self.children.push(child);
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
    fn new_file_sets_is_dir_false() {
        let node = FsNode::new_file("file.txt", "/tmp/file.txt", 1024);
        assert!(!node.is_dir);
        assert!(node.is_file());
        assert_eq!(node.size, 1024);
    }

    #[test]
    fn new_dir_sets_is_dir_true() {
        let node = FsNode::new_dir("docs", "/home/user/docs");
        assert!(node.is_dir);
        assert!(!node.is_file());
    }

    #[test]
    fn size_delta_returns_none_without_prev_size() {
        let node = FsNode::new_file("a.txt", "/a.txt", 500);
        assert_eq!(node.size_delta(), None);
    }

    #[test]
    fn size_delta_computes_correctly() {
        let mut node = FsNode::new_file("a.txt", "/a.txt", 500);
        node.prev_size = Some(300);
        assert_eq!(node.size_delta(), Some(200));
    }

    #[test]
    fn add_child_updates_counts_and_size() {
        let mut parent = FsNode::new_dir("root", "/root");
        let child_file = FsNode::new_file("file.txt", "/root/file.txt", 100);
        parent.add_child(child_file);
        assert_eq!(parent.file_count, 1);
        assert_eq!(parent.folder_count, 0);
        assert_eq!(parent.size, 100);
    }

    #[test]
    fn serialization_round_trip() {
        let node = FsNode::new_file("test.rs", "/src/test.rs", 2048);
        let json = serde_json::to_string(&node).unwrap();
        let restored: FsNode = serde_json::from_str(&json).unwrap();
        assert_eq!(node, restored);
    }
}