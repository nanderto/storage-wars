//! Flat database representation of a filesystem node.

use serde::{Deserialize, Serialize};

/// A flat representation of a filesystem node suitable for storage in a
/// relational database. Uses a parent ID to reconstruct the tree hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DbNode {
    /// Unique identifier for this node within the database.
    pub id: i64,

    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<i64>,

    /// Identifier of the scan session this node belongs to.
    pub scan_id: i64,

    /// Display name of the file or directory.
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Size in bytes.
    pub size: u64,

    /// Size in bytes from the previous scan session.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node.
    pub file_count: u64,

    /// Number of folders contained within this node.
    pub folder_count: u64,

    /// Whether this node represents a directory.
    pub is_dir: bool,

    /// Last modification timestamp as a Unix timestamp (seconds since epoch).
    pub modified_secs: Option<i64>,
}

impl DbNode {
    /// Returns `true` if this node is a root node (has no parent).
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(id: i64, parent_id: Option<i64>) -> DbNode {
        DbNode {
            id,
            parent_id,
            scan_id: 1,
            name: "test".to_string(),
            path: "/test".to_string(),
            size: 0,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            is_dir: true,
            modified_secs: None,
        }
    }

    #[test]
    fn root_node_has_no_parent() {
        let node = make_node(1, None);
        assert!(node.is_root());
    }

    #[test]
    fn child_node_has_parent() {
        let node = make_node(2, Some(1));
        assert!(!node.is_root());
    }
}