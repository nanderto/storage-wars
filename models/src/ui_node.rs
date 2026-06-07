//! UI-layer wrapper around `FsNode` with display state.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// A wrapper around `FsNode` that carries additional state needed by the UI
/// layer, such as tree depth, expansion state, and scan progress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    /// The underlying filesystem node data.
    pub node: FsNode,

    /// Depth of this node in the displayed tree (root = 0).
    pub depth: u32,

    /// Whether this directory node is currently expanded in the tree view.
    pub is_expanded: bool,

    /// Scan progress for this node as a value between 0.0 and 1.0.
    /// `None` if the node is not currently being scanned.
    pub scan_progress: Option<f32>,
}

impl UiNode {
    /// Creates a new `UiNode` wrapping the given `FsNode` at the specified depth.
    pub fn new(node: FsNode, depth: u32) -> Self {
        Self {
            node,
            depth,
            is_expanded: false,
            scan_progress: None,
        }
    }

    /// Creates a new `UiNode` that is pre-expanded.
    pub fn new_expanded(node: FsNode, depth: u32) -> Self {
        Self {
            node,
            depth,
            is_expanded: true,
            scan_progress: None,
        }
    }

    /// Toggles the expansion state of this node.
    pub fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    /// Sets the scan progress, clamping the value to [0.0, 1.0].
    pub fn set_scan_progress(&mut self, progress: f32) {
        self.scan_progress = Some(progress.clamp(0.0, 1.0));
    }

    /// Clears the scan progress, indicating the scan for this node is done.
    pub fn clear_scan_progress(&mut self) {
        self.scan_progress = None;
    }

    /// Returns `true` if this node is currently being scanned.
    pub fn is_scanning(&self) -> bool {
        self.scan_progress.is_some()
    }

    /// Returns the display name of the underlying node.
    pub fn name(&self) -> &str {
        &self.node.name
    }

    /// Returns the path of the underlying node.
    pub fn path(&self) -> &str {
        &self.node.path
    }
}

impl From<FsNode> for UiNode {
    fn from(node: FsNode) -> Self {
        UiNode::new(node, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_node() -> FsNode {
        FsNode::new_dir("documents", "/home/user/documents")
    }

    #[test]
    fn test_new_ui_node() {
        let ui = UiNode::new(sample_node(), 2);
        assert_eq!(ui.depth, 2);
        assert!(!ui.is_expanded);
        assert!(ui.scan_progress.is_none());
    }

    #[test]
    fn test_toggle_expanded() {
        let mut ui = UiNode::new(sample_node(), 0);
        assert!(!ui.is_expanded);
        ui.toggle_expanded();
        assert!(ui.is_expanded);
        ui.toggle_expanded();
        assert!(!ui.is_expanded);
    }

    #[test]
    fn test_set_scan_progress_clamped() {
        let mut ui = UiNode::new(sample_node(), 0);
        ui.set_scan_progress(1.5);
        assert_eq!(ui.scan_progress, Some(1.0));
        ui.set_scan_progress(-0.5);
        assert_eq!(ui.scan_progress, Some(0.0));
        ui.set_scan_progress(0.75);
        assert_eq!(ui.scan_progress, Some(0.75));
    }

    #[test]
    fn test_clear_scan_progress() {
        let mut ui = UiNode::new(sample_node(), 0);
        ui.set_scan_progress(0.5);
        assert!(ui.is_scanning());
        ui.clear_scan_progress();
        assert!(!ui.is_scanning());
    }

    #[test]
    fn test_from_fs_node() {
        let fs = sample_node();
        let ui: UiNode = fs.clone().into();
        assert_eq!(ui.depth, 0);
        assert_eq!(ui.node, fs);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let ui = UiNode::new(sample_node(), 1);
        let json = serde_json::to_string(&ui).expect("serialization failed");
        let restored: UiNode = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(ui, restored);
    }
}