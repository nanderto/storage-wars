//! UI-layer wrapper around [`FsNode`] with display state.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Wraps an [`FsNode`] with additional state required by the UI layer,
/// such as tree depth, expansion state, and scan progress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    /// The underlying filesystem node data.
    pub node: FsNode,

    /// Depth of this node in the displayed tree (root = 0).
    pub depth: u32,

    /// Whether this directory node is currently expanded in the tree view.
    pub expanded: bool,

    /// Scan progress for this node as a value between `0.0` and `1.0`.
    /// `None` if no scan is in progress for this node.
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

    /// Returns `true` if this node is currently being scanned.
    pub fn is_scanning(&self) -> bool {
        self.scan_progress.is_some()
    }

    /// Returns `true` if this node can be expanded (is a non-empty directory).
    pub fn is_expandable(&self) -> bool {
        self.node.is_dir && !self.node.children.is_empty()
    }

    /// Toggles the expanded state of this node.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ui_node_is_collapsed() {
        let fs_node = FsNode::new_dir("src", "/project/src");
        let ui_node = UiNode::new(fs_node, 1);
        assert!(!ui_node.expanded);
        assert_eq!(ui_node.depth, 1);
        assert!(ui_node.scan_progress.is_none());
    }

    #[test]
    fn toggle_expanded_flips_state() {
        let fs_node = FsNode::new_dir("src", "/project/src");
        let mut ui_node = UiNode::new(fs_node, 0);
        ui_node.toggle_expanded();
        assert!(ui_node.expanded);
        ui_node.toggle_expanded();
        assert!(!ui_node.expanded);
    }

    #[test]
    fn is_expandable_requires_children() {
        let mut fs_node = FsNode::new_dir("src", "/project/src");
        let ui_node = UiNode::new(fs_node.clone(), 0);
        assert!(!ui_node.is_expandable());

        fs_node.children.push(FsNode::new_file("main.rs", "/project/src/main.rs", 512));
        let ui_node_with_child = UiNode::new(fs_node, 0);
        assert!(ui_node_with_child.is_expandable());
    }
}