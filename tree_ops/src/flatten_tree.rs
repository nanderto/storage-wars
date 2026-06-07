//! Converts a nested `FsNode` hierarchy into a flat `Vec<UiNode>` for UI rendering.

use std::collections::HashSet;
use std::path::PathBuf;
use crate::models::{FsNode, UiNode};

/// Flattens a forest of `FsNode` trees into a `Vec<UiNode>` suitable for UI display.
///
/// Only children of nodes whose paths are in `expanded_paths` are included.
/// The `scan_progress` field of each `UiNode` is set to the node's size divided
/// by the maximum sibling size (fraction of the largest sibling), or `1.0` if
/// there are no siblings or the max size is zero.
///
/// # Arguments
/// * `roots` - The root nodes of the forest to flatten.
/// * `expanded_paths` - Set of paths that are currently expanded in the UI.
///
/// # Returns
/// A flat `Vec<UiNode>` in pre-order traversal order.
pub fn flatten_tree(roots: &[FsNode], expanded_paths: &HashSet<PathBuf>) -> Vec<UiNode> {
    let mut result = Vec::new();
    flatten_siblings(roots, expanded_paths, 0, &mut result);
    result
}

fn flatten_siblings(
    siblings: &[FsNode],
    expanded_paths: &HashSet<PathBuf>,
    depth: usize,
    result: &mut Vec<UiNode>,
) {
    if siblings.is_empty() {
        return;
    }

    // Compute the maximum size among siblings for scan_progress calculation.
    let max_size = siblings.iter().map(|n| n.size).max().unwrap_or(0);

    for node in siblings {
        let is_expanded = expanded_paths.contains(&node.path);

        let scan_progress = if max_size == 0 {
            1.0_f64
        } else {
            node.size as f64 / max_size as f64
        };

        result.push(UiNode {
            id: node.id,
            path: node.path.clone(),
            size: node.size,
            item_count: node.item_count,
            is_dir: node.is_dir,
            depth,
            is_expanded,
            scan_progress,
            prev_size: None,
        });

        // Recurse into children only if this node is expanded.
        if is_expanded && !node.children.is_empty() {
            flatten_siblings(&node.children, expanded_paths, depth + 1, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_fs_node(id: u64, path: &str, size: u64, children: Vec<FsNode>) -> FsNode {
        FsNode {
            id,
            path: PathBuf::from(path),
            size,
            item_count: children.len() as u64,
            is_dir: !children.is_empty(),
            children,
        }
    }

    #[test]
    fn test_empty_roots() {
        let result = flatten_tree(&[], &HashSet::new());
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_root_not_expanded() {
        let roots = vec![make_fs_node(1, "/root", 100, vec![
            make_fs_node(2, "/root/a", 50, vec![]),
        ])];
        let expanded = HashSet::new();
        let result = flatten_tree(&roots, &expanded);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert!(!result[0].is_expanded);
    }

    #[test]
    fn test_expanded_shows_children() {
        let roots = vec![make_fs_node(1, "/root", 100, vec![
            make_fs_node(2, "/root/a", 60, vec![]),
            make_fs_node(3, "/root/b", 40, vec![]),
        ])];
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));

        let result = flatten_tree(&roots, &expanded);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[1].id, 2);
        assert_eq!(result[2].id, 3);
    }

    #[test]
    fn test_scan_progress_fraction() {
        let roots = vec![
            make_fs_node(1, "/a", 100, vec![]),
            make_fs_node(2, "/b", 50, vec![]),
        ];
        let result = flatten_tree(&roots, &HashSet::new());
        assert_eq!(result.len(), 2);
        assert!((result[0].scan_progress - 1.0).abs() < f64::EPSILON);
        assert!((result[1].scan_progress - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_depth_increments() {
        let roots = vec![make_fs_node(1, "/root", 100, vec![
            make_fs_node(2, "/root/a", 100, vec![
                make_fs_node(3, "/root/a/b", 100, vec![]),
            ]),
        ])];
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));
        expanded.insert(PathBuf::from("/root/a"));

        let result = flatten_tree(&roots, &expanded);
        assert_eq!(result[0].depth, 0);
        assert_eq!(result[1].depth, 1);
        assert_eq!(result[2].depth, 2);
    }

    #[test]
    fn test_zero_size_siblings() {
        let roots = vec![
            make_fs_node(1, "/a", 0, vec![]),
            make_fs_node(2, "/b", 0, vec![]),
        ];
        let result = flatten_tree(&roots, &HashSet::new());
        assert!((result[0].scan_progress - 1.0).abs() < f64::EPSILON);
        assert!((result[1].scan_progress - 1.0).abs() < f64::EPSILON);
    }
}