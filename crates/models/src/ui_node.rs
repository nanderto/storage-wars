use crate::FsNode;

/// Wrapper around [`FsNode`] carrying UI-specific state for tree rendering.
#[derive(Clone, Debug, PartialEq)]
pub struct UiNode {
    /// The underlying filesystem node data.
    pub node: FsNode,
    /// Depth level in the visible tree (0 = root).
    pub depth: u32,
    /// Whether this directory node is currently expanded in the UI.
    pub expanded: bool,
    /// Optional scan progress percentage (0.0–1.0) while a scan is active.
    pub scan_progress: Option<f64>,
}