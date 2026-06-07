//! UI-layer wrapper around `FsNode` with display state.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// A wrapper around [`FsNode`] that carries additional UI state for rendering
/// in a tree view component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    /// The underlying filesystem node data.
    pub node: FsNode,

    /// Depth of this node in the visible tree (root = 0).
    pub depth: u32,

    /// Whether this directory node is currently expanded in the UI.
    /// Always `false` for file nodes.
    pub expanded: bool,

    /// Scan progress for this node as a value between `0.0` and `1.0`.
    /// `None` if no scan is currently active for this node.
    pub scan_progress: Option<f32>,
}

impl UiNode {
    /// Creates a new `UiNode` wrapping the given `FsNode` at the specified depth.
    pub fn new(node: FsNode, depth: u32) -> Self {
        Self {
            expanded: false,
            scan_progress: None,
            node,
            depth,
        }
    }

    /// Creates a new `UiNode` that is pre-expanded.
    pub fn new_expanded(node: FsNode, depth: u32) -> Self {
        Self {
            expanded: node.is_dir,
            scan_progress: None,
            node,
            depth,
        }
    }

    /// Toggles the expanded state. Has no effect on file nodes.
    pub fn toggle_expanded(&mut self) {
        if self.node.is_dir {
            self.expanded = !self.expanded;
        }
    }

    /// Sets the scan progress value, clamping it to `[0.0, 1.0]`.
    pub fn set_scan_progress(&mut self, progress: f32) {
        self.scan_progress = Some(progress.clamp(0.0, 1.0));
    }

    /// Clears the scan progress, indicating no active scan.
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
    use crate::FsNode;

    fn make_dir_node() -> FsNode {
        FsNode::new_dir("docs", "/docs", None)
    }

    fn make_file_node() -> FsNode {
        FsNode::new_file("readme.md", "/docs/readme.md", 1024, None)
    }

    #[test]
    fn test_new_ui_node_defaults() {
        let ui = UiNode::new(make_dir_node(), 0);
        assert_eq!(ui.depth, 0);
        assert!(!ui.expanded);
        assert!(ui.scan_progress.is_none());
    }

    #[test]
    fn test_toggle_expanded_dir() {
        let mut ui = UiNode::new(make_dir_node(), 0);
        ui.toggle_expanded();
        assert!(ui.expanded);
        ui.toggle_expanded();
        assert!(!ui.expanded);
    }

    #[test]
    fn test_toggle_expanded_file_no_effect() {
        let mut ui = UiNode::new(make_file_node(), 1);
        ui.toggle_expanded();
        assert!(!ui.expanded);
    }

    #[test]
    fn test_set_scan_progress_clamped() {
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
    fn test_new_expanded() {
        let ui = UiNode::new_expanded(make_dir_node(), 0);
        assert!(ui.expanded);
    }
}