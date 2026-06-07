use crate::FsNode;

/// Wrapper around `FsNode` carrying UI-specific state.
#[derive(Clone, Debug, PartialEq)]
pub struct UiNode {
    /// The underlying filesystem node.
    pub node: FsNode,
    /// Depth in the displayed tree (0 = root).
    pub depth: u32,
    /// Whether this node's children are currently visible in the UI.
    pub expanded: bool,
    /// Optional scan progress percentage (0.0 – 1.0).
    pub scan_progress: Option<f64>,
}