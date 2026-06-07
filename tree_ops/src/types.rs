//! Core data types used throughout the tree_ops crate.

use std::path::PathBuf;

/// A flat database record representing a filesystem node.
#[derive(Debug, Clone, PartialEq)]
pub struct DbNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Filesystem path of this node.
    pub path: PathBuf,
    /// Size in bytes of this node (file size or directory total).
    pub size: u64,
    /// Number of child entries (relevant for directories).
    pub child_count: usize,
    /// Whether this node represents a directory.
    pub is_dir: bool,
}

/// A hierarchical filesystem node with nested children.
#[derive(Debug, Clone, PartialEq)]
pub struct FsNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Filesystem path of this node.
    pub path: PathBuf,
    /// Size in bytes of this node.
    pub size: u64,
    /// Total number of descendant entries.
    pub child_count: usize,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Previous size in bytes from a baseline snapshot; `None` if not set.
    pub prev_size: Option<u64>,
    /// Nested child nodes (populated for directories).
    pub children: Vec<FsNode>,
}

impl FsNode {
    /// Creates a new `FsNode` with no children and no baseline size.
    pub fn new(id: u64, path: PathBuf, size: u64, child_count: usize, is_dir: bool) -> Self {
        Self {
            id,
            path,
            size,
            child_count,
            is_dir,
            prev_size: None,
            children: Vec::new(),
        }
    }
}

/// A flattened node suitable for UI rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Filesystem path of this node.
    pub path: PathBuf,
    /// Size in bytes of this node.
    pub size: u64,
    /// Total number of descendant entries.
    pub child_count: usize,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Previous size in bytes from a baseline snapshot; `None` if not set.
    pub prev_size: Option<u64>,
    /// Depth level in the tree (root = 0).
    pub depth: usize,
    /// Whether this node is currently expanded in the UI.
    pub is_expanded: bool,
    /// Scan progress as a fraction [0.0, 1.0] relative to the largest sibling.
    pub scan_progress: f64,
}