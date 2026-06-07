//! Data models shared across tree_ops operations.

use std::path::PathBuf;

/// A flat database node as stored/retrieved from persistence.
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
    /// Number of files contained within (1 for files, N for directories).
    pub file_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
}

/// A node in the reconstructed filesystem tree hierarchy.
#[derive(Debug, Clone, PartialEq)]
pub struct FsNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Filesystem path of this node.
    pub path: PathBuf,
    /// Size in bytes of this node.
    pub size: u64,
    /// Number of files contained within.
    pub file_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Child nodes (populated for directories).
    pub children: Vec<FsNode>,
    /// Progress of an ongoing scan (0.0–1.0); `None` if scan is complete.
    pub scan_progress: Option<f64>,
}

/// A flattened node suitable for UI rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    /// Filesystem path of this node.
    pub path: PathBuf,
    /// Size in bytes of this node.
    pub size: u64,
    /// Number of files contained within.
    pub file_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Depth level in the tree (root = 0).
    pub depth: usize,
    /// Whether this directory node is currently expanded in the UI.
    pub is_expanded: bool,
    /// Scan progress as a fraction of the largest sibling (0.0–1.0).
    pub scan_progress: Option<f64>,
    /// Previous size from baseline for change detection; `None` if no baseline.
    pub prev_size: Option<u64>,
}

impl FsNode {
    /// Creates a new leaf `FsNode` with no children.
    pub fn new_leaf(id: u64, path: PathBuf, size: u64, file_count: u64, is_dir: bool) -> Self {
        Self {
            id,
            path,
            size,
            file_count,
            is_dir,
            children: Vec::new(),
            scan_progress: None,
        }
    }
}