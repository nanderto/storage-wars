//! Flat database representation of a filesystem node.

use serde::{Deserialize, Serialize};

/// A flat representation of a filesystem node suitable for storage in a
/// relational database. Uses a `parent_id` foreign key to reconstruct the
/// tree hierarchy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// Size in bytes from the previous scan session, if available.
    pub prev_size: Option<u64>,

    /// Number of files under this node.
    pub file_count: u64,

    /// Number of sub-directories under this node.
    pub folder_count: u64,

    /// Last modification timestamp as a Unix epoch second.
    pub modified: Option<i64>,

    /// Whether this node represents a directory.
    pub is_dir: bool,
}

impl DbNode {
    /// Creates a new [`DbNode`] with the given identifiers, name, and path.
    /// All numeric fields default to zero and optional fields to `None`.
    pub fn new(
        id: i64,
        parent_id: Option<i64>,
        scan_id: i64,
        name: impl Into<String>,
        path: impl Into<String>,
        is_dir: bool,
    ) -> Self {
        Self {
            id,
            parent_id,
            scan_id,
            name: name.into(),
            path: path.into(),
            size: 0,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            modified: None,
            is_dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_db_node_defaults_are_zero() {
        let node = DbNode::new(1, None, 42, "root", "/", true);
        assert_eq!(node.id, 1);
        assert_eq!(node.parent_id, None);
        assert_eq!(node.scan_id, 42);
        assert_eq!(node.size, 0);
        assert!(node.is_dir);
    }
}