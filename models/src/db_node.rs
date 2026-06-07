//! Flat database representation of a filesystem node.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A flat representation of a filesystem node suitable for storage in a relational database.
///
/// Unlike [`crate::FsNode`], which forms a tree via nested children, `DbNode` uses
/// a `parent_id` foreign key to express the parent–child relationship.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbNode {
    /// Unique identifier for this node within the database.
    pub id: i64,

    /// Identifier of the parent node. `None` for root nodes.
    pub parent_id: Option<i64>,

    /// Identifier of the scan session this node belongs to.
    pub scan_id: i64,

    /// Display name of the file or directory.
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Size in bytes.
    pub size: u64,

    /// Previous size in bytes from the prior scan session, if available.
    pub prev_size: Option<u64>,

    /// Number of files under this node (recursive).
    pub file_count: u64,

    /// Number of folders under this node (recursive).
    pub folder_count: u64,

    /// Last modified timestamp.
    pub modified: Option<DateTime<Utc>>,

    /// Whether this node represents a directory.
    pub is_dir: bool,
}

impl DbNode {
    /// Creates a new `DbNode` with the given identifiers and basic metadata.
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
            modified: None,
            is_dir,
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
        let node = DbNode::new(1, None, 10, "root", "/", 0, true);
        assert_eq!(node.id, 1);
        assert!(node.parent_id.is_none());
        assert_eq!(node.scan_id, 10);
        assert!(node.is_dir);
        assert!(node.is_root());
    }

    #[test]
    fn test_non_root_node() {
        let node = DbNode::new(2, Some(1), 10, "docs", "/docs", 4096, true);
        assert!(!node.is_root());
        assert_eq!(node.parent_id, Some(1));
    }

    #[test]
    fn test_file_node() {
        let node = DbNode::new(3, Some(2), 10, "readme.md", "/docs/readme.md", 512, false);
        assert!(!node.is_dir);
        assert_eq!(node.size, 512);
    }
}