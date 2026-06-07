//! Core data types used throughout the tree_ops crate.

use std::path::PathBuf;

/// A flat database record representing a filesystem entry.
#[derive(Debug, Clone, PartialEq)]
pub struct DbNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Display name of the filesystem entry.
    pub name: String,
    /// Absolute path to the filesystem entry.
    pub path: PathBuf,
    /// Size in bytes of this entry (file size or directory total).
    pub size: u64,
    /// Number of direct children (for directories).
    pub child_count: usize,
    /// Whether this entry is a directory.
    pub is_dir: bool,
}

/// A node in the reconstructed filesystem tree hierarchy.
#[derive(Debug, Clone, PartialEq)]
pub struct FsNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Display name of the filesystem entry.
    pub name: String,
    /// Absolute path to the filesystem entry.
    pub path: PathBuf,
    /// Size in bytes of this entry.
    pub size: u64,
    /// Total number of descendant file entries.
    pub file_count: usize,
    /// Whether this entry is a directory.
    pub is_dir: bool,
    /// Child nodes nested under this node.
    pub children: Vec<FsNode>,
}

/// A flattened node ready for UI rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Display name of the filesystem entry.
    pub name: String,
    /// Absolute path to the filesystem entry.
    pub path: PathBuf,
    /// Size in bytes of this entry.
    pub size: u64,
    /// Size from a previous scan baseline, if available.
    pub prev_size: Option<u64>,
    /// Total number of descendant file entries.
    pub file_count: usize,
    /// Whether this entry is a directory.
    pub is_dir: bool,
    /// Depth level in the tree (root = 0).
    pub depth: usize,
    /// Whether this node is currently expanded in the UI.
    pub is_expanded: bool,
    /// Scan progress as a fraction [0.0, 1.0] relative to the largest sibling.
    pub scan_progress: f64,
}

impl FsNode {
    /// Creates a new [`FsNode`] from a [`DbNode`] with no children.
    pub fn from_db_node(db: DbNode) -> Self {
        Self {
            id: db.id,
            parent_id: db.parent_id,
            name: db.name,
            path: db.path,
            size: db.size,
            file_count: db.child_count,
            is_dir: db.is_dir,
            children: Vec::new(),
        }
    }
}