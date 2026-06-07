//! UI-layer wrapper around [`FsNode`] with display state.

use serde::{Deserialize, Serialize};

use crate::fs_node::FsNode;

/// A wrapper around [`FsNode`] that carries additional state needed by the
/// user interface, such as tree depth, expansion state, and scan progress.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiNode {
    /// The underlying filesystem node data.
    pub node: FsNode,

    /// Depth of this node in the displayed tree (root = 0).
    pub depth: usize,

    /// Whether this directory node is currently expanded in the tree view.
    pub expanded: bool,

    /// Scan progress for this node expressed as a value in `[0.0, 1.0]`.
    /// `None` when no scan is in progress for this node.
    pub scan_progress: Option<f32>,
}

impl UiNode {
    /// Creates a new [`UiNode`] wrapping the given [`FsNode`] at the specified
    /// `depth`. The node starts collapsed with no scan progress.
    pub fn new(node: FsNode, depth: usize) -> Self {
        Self {
            node,
            depth,
            expanded: false,
            scan_progress: None,
        }
    }

    /// Convenience accessor for the underlying node's path.
    pub fn path(&self) -> &str {
        &self.node.path
    }

    /// Convenience accessor for the underlying node's display name.
    pub fn name(&self) -> &str {
        &self.node.name
    }

    /// Returns `true` if this node represents a directory.
    pub fn is_dir(&self) -> bool {
        self.node.is_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fs_node() -> FsNode {
        FsNode::new("docs", "/home/user/docs", true)
    }

    #[test]
    fn new_ui_node_is_collapsed() {
        let ui = UiNode::new(make_fs_node(), 2);
        assert_eq!(ui.depth, 2);
        assert!(!ui.expanded);
        assert_eq!(ui.scan_progress, None);
    }

    #[test]
    fn accessors_delegate_to_inner_node() {
        let ui = UiNode::new(make_fs_node(), 0);
        assert_eq!(ui.name(), "docs");
        assert_eq!(ui.path(), "/home/user/docs");
        assert!(ui.is_dir());
    }
}