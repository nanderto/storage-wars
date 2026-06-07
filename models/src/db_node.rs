//! Flat database representation of a filesystem node.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A flat representation of a filesystem node suitable for storage in a
/// relational database. Uses a `parent_id` foreign key to reconstruct the
/// tree hierarchy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbNode {
    /// Unique identifier for this node within the database.
    pub id: i64,

    /// The identifier of the parent node. `None` for root nodes.
    pub parent_id: Option<i64>,

    /// The scan session this node belongs to.
    pub scan_id: i64,

    /// The name of the file or directory.
    pub name: String,

    /// The absolute path of the file or directory.
    pub path: String,

    /// The current size in bytes.
    pub size: u64,

    /// The size in bytes from the previous scan, if available.
    pub prev_size: Option<u64>,

    /// The number of files contained within this node.
    pub file_count: u64,

    /// The number of folders contained within this node.
    pub folder_count: u64,

    /// Whether this node represents a file (`true`) or a directory (`false`).
    pub is_file: bool,

    /// The last-modified timestamp of the file or directory.
    pub modified: Option<DateTime<Utc>>,
}

impl DbNode {
    /// Creates a new [`DbNode`] with the given identifiers and metadata.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        parent_id: Option<i64>,
        scan_id: i64,
        name: impl Into<String>,
        path: impl Into<String>,
        size: u64,
        is_file: bool,
        modified: Option<DateTime<Utc>>,
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
            is_file,
            modified,
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
    fn is_root_true_when_no_parent() {
        let node = DbNode::new(1, None, 10, "root", "/", 0, false, None);
        assert!(node.is_root());
    }

    #[test]
    fn is_root_false_when_has_parent() {
        let node = DbNode::new(2, Some(1), 10, "child", "/child", 100, true, None);
        assert!(!node.is_root());
    }
}