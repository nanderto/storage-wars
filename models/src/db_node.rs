//! Flat database representation of a filesystem node.

use serde::{Deserialize, Serialize};

/// A flat representation of a filesystem node suitable for storage in a
/// relational database. Uses a `parent_id` foreign key to reconstruct the
/// tree hierarchy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbNode {
    /// Unique identifier for this node within a scan session.
    pub id: i64,

    /// Identifier of the parent node. `None` for root nodes.
    pub parent_id: Option<i64>,

    /// Identifier of the scan session this node belongs to.
    pub scan_id: i64,

    /// Display name of the file or directory.
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Total size in bytes.
    pub size: u64,

    /// Size in bytes from the previous scan, if available.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node.
    pub file_count: u64,

    /// Number of folders contained within this node.
    pub folder_count: u64,

    /// Whether this node represents a directory.
    pub is_dir: bool,

    /// Last modification timestamp as a Unix timestamp (seconds).
    pub modified_secs: Option<i64>,
}

impl DbNode {
    /// Creates a new `DbNode` with the given identifiers and path information.
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
            modified_secs: None,
        }
    }

    /// Returns `true` if this node is a root node (has no parent).
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_db_node() {
        let node = DbNode::new(1, None, 42, "root", "/", 0, true);
        assert_eq!(node.id, 1);
        assert!(node.parent_id.is_none());
        assert_eq!(node.scan_id, 42);
        assert!(node.is_dir);
        assert!(node.is_root());
    }

    #[test]
    fn test_child_db_node() {
        let node = DbNode::new(2, Some(1), 42, "docs", "/docs", 4096, true);
        assert!(!node.is_root());
        assert_eq!(node.parent_id, Some(1));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let node = DbNode::new(5, Some(1), 10, "file.txt", "/file.txt", 256, false);
        let json = serde_json::to_string(&node).expect("serialization failed");
        let restored: DbNode = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(node, restored);
    }
}