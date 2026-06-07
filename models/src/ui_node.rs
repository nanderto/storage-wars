//! UI-layer wrapper around [`FsNode`] with display state.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Wraps an [`FsNode`] with additional state required by the UI layer,
/// such as tree depth, expansion state, and optional scan progress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    /// The underlying filesystem node data.
    pub node: FsNode,

    /// Depth of this node in the displayed tree (root = 0).
    pub depth: u32,

    /// Whether this directory node is currently expanded in the tree view.
    pub is_expanded: bool,

    /// Scan progress for this node as a value in `[0.0, 1.0]`, or `None`
    /// if scanning is not in progress for this node.
    pub scan_progress: Option<f32>,
}

impl UiNode {
    /// Creates a new [`UiNode`] wrapping the given [`FsNode`] at the
    /// specified tree `depth`.  The node starts collapsed with no scan
    /// progress.
    pub fn new(node: FsNode, depth: u32) -> Self {
        Self {
            node,
            depth,
            is_expanded: false,
            scan_progress: None,
        }
    }

    /// Toggles the expansion state of this node.
    ///
    /// Has no effect if the underlying node is not a directory.
    pub fn toggle_expanded(&mut self) {
        if self.node.is_dir {
            self.is_expanded = !self.is_expanded;
        }
    }

    /// Sets the scan progress, clamping the value to `[0.0, 1.0]`.
    pub fn set_scan_progress(&mut self, progress: f32) {
        self.scan_progress = Some(progress.clamp(0.0, 1.0));
    }

    /// Clears the scan progress, indicating that scanning has finished or
    /// has not started.
    pub fn clear_scan_progress(&mut self) {
        self.scan_progress = None;
    }

    /// Returns `true` if this node is currently being scanned.
    pub fn is_scanning(&self) -> bool {
        self.scan_progress.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dir_node() -> FsNode {
        let mut n = FsNode::new("docs", "/home/user/docs");
        n.is_dir = true;
        n
    }

    fn make_file_node() -> FsNode {
        FsNode::new("readme.txt", "/home/user/readme.txt")
    }

    #[test]
    fn test_new_ui_node_defaults() {
        let ui = UiNode::new(make_dir_node(), 1);
        assert_eq!(ui.depth, 1);
        assert!(!ui.is_expanded);
        assert!(ui.scan_progress.is_none());
    }

    #[test]
    fn test_toggle_expanded_dir() {
        let mut ui = UiNode::new(make_dir_node(), 0);
        assert!(!ui.is_expanded);
        ui.toggle_expanded();
        assert!(ui.is_expanded);
        ui.toggle_expanded();
        assert!(!ui.is_expanded);
    }

    #[test]
    fn test_toggle_expanded_file_no_effect() {
        let mut ui = UiNode::new(make_file_node(), 0);
        ui.toggle_expanded();
        assert!(!ui.is_expanded);
    }

    #[test]
    fn test_scan_progress_clamping() {
        let mut ui = UiNode::new(make_dir_node(), 0);
        ui.set_scan_progress(1.5);
        assert_eq!(ui.scan_progress, Some(1.0));
        ui.set_scan_progress(-0.5);
        assert_eq!(ui.scan_progress, Some(0.0));
    }

    #[test]
    fn test_clear_scan_progress() {
        let mut ui = UiNode::new(make_dir_node(), 0);
        ui.set_scan_progress(0.5);
        assert!(ui.is_scanning());
        ui.clear_scan_progress();
        assert!(!ui.is_scanning());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut ui = UiNode::new(make_dir_node(), 2);
        ui.is_expanded = true;
        ui.set_scan_progress(0.75);

        let json = serde_json::to_string(&ui).expect("serialization failed");
        let restored: UiNode = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(ui, restored);
    }
}