//! Core data models used throughout the tree_ops crate.

use std::path::PathBuf;

/// A flat database node as stored/retrieved from persistence.
#[derive(Debug, Clone, PartialEq)]
pub struct DbNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Filesystem path for this node.
    pub path: PathBuf,
    /// Size in bytes of this node (file size or directory total).
    pub size: u64,
    /// Number of child items (files + directories) under this node.
    pub item_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
}

/// A hierarchical filesystem node with nested children.
#[derive(Debug, Clone, PartialEq)]
pub struct FsNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Filesystem path for this node.
    pub path: PathBuf,
    /// Size in bytes of this node.
    pub size: u64,
    /// Number of child items under this node.
    pub item_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Direct children of this node.
    pub children: Vec<FsNode>,
}

/// A flattened UI node ready for display in a list or tree view.
#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Filesystem path for this node.
    pub path: PathBuf,
    /// Size in bytes of this node.
    pub size: u64,
    /// Number of child items under this node.
    pub item_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Depth level in the tree (0 = root children).
    pub depth: usize,
    /// Whether this node is currently expanded in the UI.
    pub is_expanded: bool,
    /// Scan progress as a fraction [0.0, 1.0] relative to the largest sibling.
    pub scan_progress: f64,
    /// Previous size from baseline for change detection; `None` if not in baseline.
    pub prev_size: Option<u64>,
}