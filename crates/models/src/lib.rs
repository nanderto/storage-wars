use std::path::PathBuf;

/// Represents a node in the scanned filesystem tree.
#[derive(Debug, Clone)]
pub struct FsNode {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub children: Vec<FsNode>,
    pub path: PathBuf,
}

/// Represents a flattened view-layer node used for UI rendering.
#[derive(Debug, Clone)]
pub struct UiNode {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub depth: usize,
    pub scan_progress: f64,
    pub path: PathBuf,
    pub has_children: bool,
    pub is_expanded: bool,
}