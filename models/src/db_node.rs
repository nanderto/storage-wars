//! Flat database representation of a filesystem node.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A flat representation of a filesystem node suitable for storage in a relational database.
///
/// Unlike [`crate::FsNode`], which forms a tree via nested children, [`DbNode`] uses a
/// `parent_id` foreign key to express the parent–child relationship, enabling efficient
/// storage and querying in a flat table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbNode {
    /// Unique identifier for this node within the database.
    pub id: i64,

    /// The identifier of the parent node, or `None` for root nodes.
    pub parent_id: Option<i64>,

    /// The scan session this node belongs to.
    pub scan_id: i64,

    /// The name of the file or directory.
    pub name: String,

    /// The absolute path to this node.
    pub path: String,

    /// Total size in bytes.
    pub size: u64,

    /// Size in bytes from the previous scan session.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node (recursive).
    pub file_count: u64,

    /// Number of folders contained within this node (recursive).
    pub folder_count: u64,

    /// Whether this node is a directory.
    pub is_dir: bool,

    /// Last-modified timestamp from the filesystem.
    pub modified: Option<DateTime<Utc>>,
}

impl DbNode {
    /// Creates a new [`DbNode`] with the given identifiers and basic metadata.
    pub fn new(
        id: i64,
        parent_id: Option<i64>,
        scan_id: i64,
        name: impl Into<String>,
        path: impl Into<String>,
        size: u64,
        is_dir: bool,
    ) -> Self {
        Self {
            id,
            parent_id,
            scan_id,
            name: name.into(),
            path: path.into(),
            size,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            is_dir,
            modified: None,
        }
    }

    /// Returns `true` if this node is a root node (has no parent).
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// Returns the size delta between the current and previous scan.
    pub fn size_delta(&self) -> Option<i64> {
        self.prev_size
            .map(|prev| self.size as i64 - prev as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_root_true_when_no_parent() {
        let node = DbNode::new(1, None, 10, "root", "/", 0, true);
        assert!(node.is_root());
    }

    #[test]
    fn is_root_false_when_has_parent() {
        let node = DbNode::new(2, Some(1), 10, "docs", "/docs", 0, true);
        assert!(!node.is_root());
    }

    #[test]
    fn size_delta_none_without_prev() {
        let node = DbNode::new(1, None, 1, "f", "/f", 100, false);
        assert_eq!(node.size_delta(), None);
    }

    #[test]
    fn size_delta_computed() {
        let mut node = DbNode::new(1, None, 1, "f", "/f", 200, false);
        node.prev_size = Some(150);
        assert_eq!(node.size_delta(), Some(50));
    }

    #[test]
    fn serialization_round_trip() {
        let node = DbNode::new(42, Some(1), 5, "file.txt", "/home/file.txt", 4096, false);
        let json = serde_json::to_string(&node).unwrap();
        let restored: DbNode = serde_json::from_str(&json).unwrap();
        assert_eq!(node, restored);
    }
}