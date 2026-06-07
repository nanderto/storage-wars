//! UI-layer wrapper around [`FsNode`] with display state.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Wraps an [`FsNode`] with additional state required by the UI layer,
/// such as the node's depth in the visible tree, whether it is expanded,
/// and an optional scan progress value for directories being scanned.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    /// The underlying filesystem node.
    pub node: FsNode,

    /// The depth of this node in the displayed tree (root = 0).
    pub depth: usize,

    /// Whether this directory node is expanded in the tree view.
    /// Always `false` for file nodes.
    pub expanded: bool,

    /// Scan progress for this directory, expressed as a value between
    /// `0.0` (not started) and `1.0` (complete). `None` if not being scanned.
    pub scan_progress: Option<f32>,
}

impl UiNode {
    /// Creates a new [`UiNode`] wrapping the given [`FsNode`] at the specified depth.
    pub fn new(node: FsNode, depth: usize) -> Self {
        Self {
            node,
            depth,
            expanded: false,
            scan_progress: None,
        }
    }

    /// Returns `true` if this node is currently being scanned.
    pub fn is_scanning(&self) -> bool {
        self.scan_progress.is_some()
    }

    /// Sets the scan progress, clamping the value to `[0.0, 1.0]`.
    pub fn set_scan_progress(&mut self, progress: f32) {
        self.scan_progress = Some(progress.clamp(0.0, 1.0));
    }

    /// Clears the scan progress, indicating the scan has finished.
    pub fn clear_scan_progress(&mut self) {
        self.scan_progress = None;
    }

    /// Toggles the expanded state of this node.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ui_node() -> UiNode {
        UiNode::new(FsNode::new_dir("docs", "/docs", None), 1)
    }

    #[test]
    fn new_node_is_collapsed() {
        let ui = make_ui_node();
        assert!(!ui.expanded);
    }

    #[test]
    fn toggle_expanded_flips_state() {
        let mut ui = make_ui_node();
        ui.toggle_expanded();
        assert!(ui.expanded);
        ui.toggle_expanded();
        assert!(!ui.expanded);
    }

    #[test]
    fn scan_progress_clamped() {
        let mut ui = make_ui_node();
        ui.set_scan_progress(1.5);
        assert_eq!(ui.scan_progress, Some(1.0));
        ui.set_scan_progress(-0.5);
        assert_eq!(ui.scan_progress, Some(0.0));
    }

    #[test]
    fn clear_scan_progress_removes_value() {
        let mut ui = make_ui_node();
        ui.set_scan_progress(0.5);
        ui.clear_scan_progress();
        assert!(!ui.is_scanning());
    }
}