//! Data models used throughout the tree_ops crate.

use std::collections::HashMap;
use std::path::PathBuf;

/// A flat database node as stored or retrieved from persistent storage.
#[derive(Debug, Clone, PartialEq)]
pub struct DbNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Display name of the node (file or directory name).
    pub name: String,
    /// Absolute filesystem path.
    pub path: PathBuf,
    /// Size in bytes. For directories this may be `0` until recalculated.
    pub size: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Number of direct children (files + directories).
    pub child_count: usize,
}

/// A hierarchical filesystem node used for in-memory tree operations.
#[derive(Debug, Clone, PartialEq)]
pub struct FsNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Display name of the node (file or directory name).
    pub name: String,
    /// Absolute filesystem path.
    pub path: PathBuf,
    /// Size in bytes.
    pub size: u64,
    /// Previous size in bytes (populated from a baseline map).
    pub prev_size: Option<u64>,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Total number of descendant file entries.
    pub file_count: usize,
    /// Direct children of this node.
    pub children: Vec<FsNode>,
}

impl FsNode {
    /// Creates a new leaf `FsNode` with no children.
    pub fn new(id: u64, name: impl Into<String>, path: PathBuf, size: u64, is_dir: bool) -> Self {
        Self {
            id,
            name: name.into(),
            path,
            size,
            prev_size: None,
            is_dir,
            file_count: 0,
            children: Vec::new(),
        }
    }
}

/// A flattened node ready for UI rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    /// Unique identifier matching the originating `FsNode`.
    pub id: u64,
    /// Display name.
    pub name: String,
    /// Absolute filesystem path.
    pub path: PathBuf,
    /// Size in bytes.
    pub size: u64,
    /// Previous size in bytes for delta display.
    pub prev_size: Option<u64>,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Depth level in the tree (root = 0).
    pub depth: usize,
    /// Whether this directory node is currently expanded in the UI.
    pub is_expanded: bool,
    /// Scan progress as a fraction `[0.0, 1.0]` relative to the largest sibling.
    pub scan_progress: f64,
    /// Total number of descendant file entries.
    pub file_count: usize,
}

/// A map from filesystem path to size in bytes, used as a baseline snapshot.
pub type BaselineMap = HashMap<PathBuf, u64>;