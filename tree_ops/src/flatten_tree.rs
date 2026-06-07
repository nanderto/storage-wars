//! Converts a nested `FsNode` tree into a flat `Vec<UiNode>` for UI rendering.

use std::collections::HashSet;
use std::path::PathBuf;
use crate::models::{FsNode, UiNode};

/// Flattens a slice of root `FsNode` trees into a `Vec<UiNode>` suitable for
/// rendering in a list-based UI component.
///
/// # Behaviour
///
/// * Only nodes whose ancestors are all present in `expanded_paths` are included
///   (the roots themselves are always included).
/// * `scan_progress` for each node is computed as `node.size / max_sibling_size`
///   where `max_sibling_size` is the largest `size` among all siblings at the
///   same level. If all siblings have size `0`, `scan_progress` is `0.0`.
/// * `depth` starts at `0` for root nodes.
///
/// # Arguments
///
/// * `roots`          — Root nodes of the forest.
/// * `expanded_paths` — Set of paths that are currently expanded in the UI.
///
/// # Returns
///
/// A depth-first ordered `Vec<UiNode>`.
pub fn flatten_tree(roots: &[FsNode], expanded_paths: &HashSet<PathBuf>) -> Vec<UiNode> {
    let mut result = Vec::new();
    let max_sibling = max_size_among(roots);
    for node in roots {
        flatten_node(node, 0, max_sibling, expanded_paths, &mut result);
    }
    result
}

/// Returns the maximum `size` among a slice of nodes, or `1` if all are zero
/// (to avoid division by zero).
fn max_size_among(nodes: &[FsNode]) -> u64 {
    nodes.iter().map(|n| n.size).max().unwrap_or(0).max(1)
}

fn flatten_node(
    node: &FsNode,
    depth: usize,
    max_sibling_size: u64,
    expanded_paths: &HashSet<PathBuf>,
    out: &mut Vec<UiNode>,
) {
    let is_expanded = node.is_dir && expanded_paths.contains(&node.path);
    let scan_progress = node.size as f64 / max_sibling_size as f64;

    out.push(UiNode {
        id: node.id,
        name: node.name.clone(),
        path: node.path.clone(),
        size: node.size,
        prev_size: node.prev_size,
        is_dir: node.is_dir,
        depth,
        is_expanded,
        scan_progress,
        file_count: node.file_count,
    });

    if is_expanded && !node.children.is_empty() {
        let child_max = max_size_among(&node.children);
        for child in &node.children {
            flatten_node(child, depth + 1, child_max, expanded_paths, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::models::FsNode;

    fn make_node(id: u64, name: &str, size: u64, is_dir: bool, children: Vec<FsNode>) -> FsNode {
        FsNode {
            id,
            name: name.into(),
            path: PathBuf::from(format!("/{name}")),
            size,
            prev_size: None,
            is_dir,
            file_count: 0,
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
        let root = make_node(1, "root", 100, true, vec![
            make_node(2, "child", 50, false, vec![]),
        ]);
        let result = flatten_tree(&[root], &HashSet::new());
        // Root is included; child is hidden because root is not expanded.
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert!(!result[0].is_expanded);
    }

    #[test]
    fn test_expanded_shows_children() {
        let root = make_node(1, "root", 100, true, vec![
            make_node(2, "child", 50, false, vec![]),
        ]);
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));

        let result = flatten_tree(&[root], &expanded);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert!(result[0].is_expanded);
        assert_eq!(result[1].id, 2);
        assert_eq!(result[1].depth, 1);
    }

    #[test]
    fn test_scan_progress_fraction() {
        let roots = vec![
            make_node(1, "a", 50, false, vec![]),
            make_node(2, "b", 100, false, vec![]),
        ];
        let result = flatten_tree(&roots, &HashSet::new());
        assert_eq!(result.len(), 2);
        // Node with size 100 should have progress 1.0.
        let b = result.iter().find(|n| n.id == 2).unwrap();
        assert!((b.scan_progress - 1.0).abs() < f64::EPSILON);
        // Node with size 50 should have progress 0.5.
        let a = result.iter().find(|n| n.id == 1).unwrap();
        assert!((a.scan_progress - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_depth_increments() {
        let root = make_node(1, "root", 100, true, vec![
            make_node(2, "mid", 80, true, vec![
                make_node(3, "leaf", 40, false, vec![]),
            ]),
        ]);
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));
        expanded.insert(PathBuf::from("/mid"));

        let result = flatten_tree(&[root], &expanded);
        assert_eq!(result[0].depth, 0);
        assert_eq!(result[1].depth, 1);
        assert_eq!(result[2].depth, 2);
    }
}