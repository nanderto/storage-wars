//! Data models shared across tree_ops operations.

use std::collections::HashMap;
use std::path::PathBuf;

/// A flat database record representing a single filesystem node.
#[derive(Debug, Clone, PartialEq)]
pub struct DbNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Display name of the node (file or directory name).
    pub name: String,
    /// Absolute path of the node on the filesystem.
    pub path: PathBuf,
    /// Size in bytes of this node (file size, or 0 for directories before aggregation).
    pub size: u64,
    /// Number of direct or total child items (0 for files).
    pub item_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
}

/// A hierarchical filesystem node, potentially containing children.
#[derive(Debug, Clone, PartialEq)]
pub struct FsNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Identifier of the parent node; `None` for root nodes.
    pub parent_id: Option<u64>,
    /// Display name of the node.
    pub name: String,
    /// Absolute path of the node on the filesystem.
    pub path: PathBuf,
    /// Aggregated size in bytes (includes children for directories).
    pub size: u64,
    /// Total number of descendant items.
    pub item_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Child nodes (populated for directories).
    pub children: Vec<FsNode>,
    /// Previous size from baseline snapshot, if available.
    pub prev_size: Option<u64>,
}

impl FsNode {
    /// Creates a new `FsNode` from a `DbNode` with no children.
    pub fn from_db_node(db: DbNode) -> Self {
        Self {
            id: db.id,
            parent_id: db.parent_id,
            name: db.name,
            path: db.path,
            size: db.size,
            item_count: db.item_count,
            is_dir: db.is_dir,
            children: Vec::new(),
            prev_size: None,
        }
    }
}

/// A flattened node suitable for UI rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    /// Unique identifier for this node.
    pub id: u64,
    /// Display name of the node.
    pub name: String,
    /// Absolute path of the node on the filesystem.
    pub path: PathBuf,
    /// Aggregated size in bytes.
    pub size: u64,
    /// Total number of descendant items.
    pub item_count: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Depth level in the tree (0 = root children).
    pub depth: usize,
    /// Whether this node is currently expanded in the UI.
    pub is_expanded: bool,
    /// Scan progress as a fraction [0.0, 1.0] relative to the largest sibling.
    pub scan_progress: f64,
    /// Previous size from baseline snapshot, if available.
    pub prev_size: Option<u64>,
}

/// A map from filesystem path to size in bytes, used as a baseline snapshot.
pub type BaselineMap = HashMap<PathBuf, u64>;