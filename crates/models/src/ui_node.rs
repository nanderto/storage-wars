use crate::FsNode;

/// Wrapper around `FsNode` carrying additional UI state.
#[derive(Clone, Debug, PartialEq)]
pub struct UiNode {
    /// The underlying filesystem node.
    pub node: FsNode,
    /// Tree depth (0 = root).
    pub depth: u32,
    /// Whether this node's children are visible in the UI.
    pub expanded: bool,
    /// Scan progress percentage (0.0 – 1.0) while a scan is in flight.
    pub scan_progress: f64,
}