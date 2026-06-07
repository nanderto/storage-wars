//! UI-layer wrapper around [`FsNode`] with display state.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// A wrapper around [`FsNode`] that adds UI-specific state for rendering the
/// filesystem tree in the desktop application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    /// The underlying filesystem node data.
    pub node: FsNode,

    /// The depth of this node in the displayed tree (root = 0).
    pub depth: u32,

    /// Whether this directory node is currently expanded in the tree view.
    /// Always `false` for file nodes.
    pub expanded: bool,

    /// Scan progress for this node, in the range `[0.0, 1.0]`.
    ///
    /// `None` if scanning has not started or has completed for this node.
    /// Used to show per-directory progress indicators during an active scan.
    pub scan_progress: Option<f32>,
}

impl UiNode {
    /// Creates a new [`UiNode`] wrapping the given [`FsNode`] at the specified depth.
    pub fn new(node: FsNode, depth: u32) -> Self {
        Self {
            node,
            depth,
            expanded: false,
            scan_progress: None,
        }
    }

    /// Creates a new [`UiNode`] that is pre-expanded.
    pub fn new_expanded(node: FsNode, depth: u32) -> Self {
        Self {
            node,
            depth,
            expanded: true,
            scan_progress: None,
        }
    }

    /// Returns `true` if this node is a directory and is currently expanded.
    pub fn is_expanded_dir(&self) -> bool {
        self.node.is_dir && self.expanded
    }

    /// Sets the scan progress, clamping the value to `[0.0, 1.0]`.
    pub fn set_scan_progress(&mut self, progress: f32) {
        self.scan_progress = Some(progress.clamp(0.0, 1.0));
    }

    /// Clears the scan progress (e.g. when scanning is complete).
    pub fn clear_scan_progress(&mut self) {
        self.scan_progress = None;
    }

    /// Toggles the expanded state of this node.
    ///
    /// Has no effect on file nodes.
    pub fn toggle_expanded(&mut self) {
        if self.node.is_dir {
            self.expanded = !self.expanded;
        }
    }

    /// Returns the indentation level suitable for rendering (same as `depth`).
    pub fn indent_level(&self) -> u32 {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dir_node() -> FsNode {
        FsNode::new_dir("src", "/project/src")
    }

    fn make_file_node() -> FsNode {
        FsNode::new_file("main.rs", "/project/src/main.rs", 512)
    }

    #[test]
    fn new_node_is_collapsed() {
        let ui = UiNode::new(make_dir_node(), 1);
        assert!(!ui.expanded);
        assert!(ui.scan_progress.is_none());
    }

    #[test]
    fn new_expanded_node_is_expanded() {
        let ui = UiNode::new_expanded(make_dir_node(), 0);
        assert!(ui.expanded);
    }

    #[test]
    fn toggle_expanded_flips_state_for_dir() {
        let mut ui = UiNode::new(make_dir_node(), 0);
        ui.toggle_expanded();
        assert!(ui.expanded);
        ui.toggle_expanded();
        assert!(!ui.expanded);
    }

    #[test]
    fn toggle_expanded_no_effect_on_file() {
        let mut ui = UiNode::new(make_file_node(), 1);
        ui.toggle_expanded();
        assert!(!ui.expanded);
    }

    #[test]
    fn set_scan_progress_clamps_value() {
        let mut ui = UiNode::new(make_dir_node(), 0);
        ui.set_scan_progress(1.5);
        assert_eq!(ui.scan_progress, Some(1.0));
        ui.set_scan_progress(-0.5);
        assert_eq!(ui.scan_progress, Some(0.0));
    }

    #[test]
    fn clear_scan_progress_removes_value() {
        let mut ui = UiNode::new(make_dir_node(), 0);
        ui.set_scan_progress(0.5);
        ui.clear_scan_progress();
        assert!(ui.scan_progress.is_none());
    }

    #[test]
    fn is_expanded_dir_requires_both_dir_and_expanded() {
        let mut ui = UiNode::new(make_dir_node(), 0);
        assert!(!ui.is_expanded_dir());
        ui.expanded = true;
        assert!(ui.is_expanded_dir());

        let file_ui = UiNode::new(make_file_node(), 1);
        assert!(!file_ui.is_expanded_dir());
    }

    #[test]
    fn serialization_round_trip() {
        let mut ui = UiNode::new(make_dir_node(), 2);
        ui.set_scan_progress(0.75);
        let json = serde_json::to_string(&ui).unwrap();
        let restored: UiNode = serde_json::from_str(&json).unwrap();
        assert_eq!(ui, restored);
    }
}