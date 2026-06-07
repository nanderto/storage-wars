//! Data models used throughout the tree_ops crate.

use std::collections::HashSet;
use std::path::PathBuf;

/// A flat node record as stored in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct DbNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Filesystem path of this node.
    pub path: PathBuf,
    /// Size in bytes of this node (file size, or 0 for directories before aggregation).
    pub size: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Number of direct children (files/directories) under this node.
    pub child_count: usize,
    /// Scan progress as a value in `[0.0, 1.0]`; `None` if scanning is complete.
    pub scan_progress: Option<f64>,
}

/// A node in the reconstructed filesystem tree hierarchy.
#[derive(Debug, Clone, PartialEq)]
pub struct FsNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Filesystem path of this node.
    pub path: PathBuf,
    /// Aggregated size in bytes (sum of all descendant file sizes).
    pub size: u64,
    /// Total number of descendant files.
    pub file_count: usize,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Scan progress as a value in `[0.0, 1.0]`; `None` if scanning is complete.
    pub scan_progress: Option<f64>,
    /// Direct child nodes.
    pub children: Vec<FsNode>,
    /// Size before the current scan (populated from a baseline map).
    pub prev_size: Option<u64>,
}

impl FsNode {
    /// Creates a new `FsNode` with no children and no previous size.
    pub fn new(id: u64, path: PathBuf, size: u64, file_count: usize, is_dir: bool) -> Self {
        Self {
            id,
            path,
            size,
            file_count,
            is_dir,
            scan_progress: None,
            children: Vec::new(),
            prev_size: None,
        }
    }
}

/// A flattened node suitable for rendering in a UI list.
#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Filesystem path of this node.
    pub path: PathBuf,
    /// Aggregated size in bytes.
    pub size: u64,
    /// Total number of descendant files.
    pub file_count: usize,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Depth in the tree (root = 0).
    pub depth: usize,
    /// Whether this directory node is currently expanded in the UI.
    pub is_expanded: bool,
    /// Fraction of the largest sibling's size; `None` for root or if siblings have zero size.
    pub size_fraction: Option<f64>,
    /// Scan progress as a value in `[0.0, 1.0]`; `None` if scanning is complete.
    pub scan_progress: Option<f64>,
    /// Size before the current scan.
    pub prev_size: Option<u64>,
}

/// The set of paths that are currently expanded in the UI.
pub type ExpandedPaths = HashSet<PathBuf>;