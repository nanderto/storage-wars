//! Flat database representation of a filesystem node.

use serde::{Deserialize, Serialize};

/// A flat representation of a filesystem node suitable for storage in a
/// relational database.  Parent–child relationships are encoded via
/// [`parent_id`] rather than nested structures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DbNode {
    /// Unique identifier for this node within the database.
    pub id: i64,

    /// Identifier of the parent node, or `None` for root nodes.
    pub parent_id: Option<i64>,

    /// Display name of the file or directory.
    pub name: String,

    /// Absolute path to the file or directory.
    pub path: String,

    /// Current size in bytes.
    pub size: u64,

    /// Size in bytes from the previous scan session, if available.
    pub prev_size: Option<u64>,

    /// Number of files contained within this node.
    pub file_count: u64,

    /// Number of sub-folders contained within this node.
    pub folder_count: u64,

    /// Whether this node represents a directory.
    pub is_dir: bool,

    /// Unix timestamp (seconds since epoch) of the last modification, if known.
    pub modified_secs: Option<i64>,

    /// The scan session this node belongs to.
    pub scan_id: i64,
}

impl DbNode {
    /// Creates a new [`DbNode`] with the given identifiers, name, and path.
    pub fn new(
        id: i64,
        scan_id: i64,
        parent_id: Option<i64>,
        name: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            id,
            parent_id,
            name: name.into(),
            path: path.into(),
            size: 0,
            prev_size: None,
            file_count: 0,
            folder_count: 0,
            is_dir: false,
            modified_secs: None,
            scan_id,
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
        let node = DbNode::new(1, 42, None, "root", "/");
        assert_eq!(node.id, 1);
        assert_eq!(node.scan_id, 42);
        assert!(node.parent_id.is_none());
        assert!(node.is_root());
        assert_eq!(node.name, "root");
        assert_eq!(node.path, "/");
    }

    #[test]
    fn test_non_root_node() {
        let node = DbNode::new(2, 42, Some(1), "home", "/home");
        assert!(!node.is_root());
        assert_eq!(node.parent_id, Some(1));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut node = DbNode::new(3, 1, Some(1), "docs", "/home/user/docs");
        node.size = 8192;
        node.is_dir = true;

        let json = serde_json::to_string(&node).expect("serialization failed");
        let restored: DbNode = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(node, restored);
    }
}