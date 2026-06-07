//! Baseline map construction and merging for change detection.

use std::collections::HashMap;
use std::path::PathBuf;
use crate::models::{FsNode, UiNode};

/// Builds a `HashMap<PathBuf, u64>` lookup from a forest of `FsNode` trees.
///
/// Every node in the forest (recursively) is included in the map, keyed by
/// its `path` with its `size` as the value.
///
/// # Arguments
/// * `nodes` - The forest of `FsNode` trees to index.
///
/// # Returns
/// A flat map of `PathBuf → size` for all nodes in the forest.
pub fn build_baseline_map(nodes: &[FsNode]) -> HashMap<PathBuf, u64> {
    let mut map = HashMap::new();
    for node in nodes {
        collect_into_map(node, &mut map);
    }
    map
}

fn collect_into_map(node: &FsNode, map: &mut HashMap<PathBuf, u64>) {
    map.insert(node.path.clone(), node.size);
    for child in &node.children {
        collect_into_map(child, map);
    }
}

/// Populates the `prev_size` field of each `UiNode` from a baseline map.
///
/// For each `UiNode`, if its `path` exists in `baseline`, `prev_size` is set
/// to `Some(baseline_size)`. Otherwise it remains `None` (new node).
///
/// # Arguments
/// * `nodes` - Mutable slice of `UiNode` items to update.
/// * `baseline` - The baseline map produced by [`build_baseline_map`].
pub fn merge_baseline(nodes: &mut Vec<UiNode>, baseline: &HashMap<PathBuf, u64>) {
    for node in nodes.iter_mut() {
        node.prev_size = baseline.get(&node.path).copied();
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
            item_count: 0,
            is_dir: !children.is_empty(),
            children,
        }
    }

    fn make_ui_node(id: u64, path: &str, size: u64) -> UiNode {
        UiNode {
            id,
            path: PathBuf::from(path),
            size,
            item_count: 0,
            is_dir: false,
            depth: 0,
            is_expanded: false,
            scan_progress: 1.0,
            prev_size: None,
        }
    }

    #[test]
    fn test_build_baseline_map_empty() {
        let map = build_baseline_map(&[]);
        assert!(map.is_empty());
    }

    #[test]
    fn test_build_baseline_map_flat() {
        let nodes = vec![
            make_fs_node(1, "/a", 100, vec![]),
            make_fs_node(2, "/b", 200, vec![]),
        ];
        let map = build_baseline_map(&nodes);
        assert_eq!(map.len(), 2);
        assert_eq!(map[&PathBuf::from("/a")], 100);
        assert_eq!(map[&PathBuf::from("/b")], 200);
    }

    #[test]
    fn test_build_baseline_map_nested() {
        let nodes = vec![make_fs_node(1, "/root", 300, vec![
            make_fs_node(2, "/root/a", 100, vec![]),
            make_fs_node(3, "/root/b", 200, vec![]),
        ])];
        let map = build_baseline_map(&nodes);
        assert_eq!(map.len(), 3);
        assert_eq!(map[&PathBuf::from("/root")], 300);
        assert_eq!(map[&PathBuf::from("/root/a")], 100);
        assert_eq!(map[&PathBuf::from("/root/b")], 200);
    }

    #[test]
    fn test_merge_baseline_sets_prev_size() {
        let mut ui_nodes = vec![
            make_ui_node(1, "/a", 150),
            make_ui_node(2, "/b", 250),
            make_ui_node(3, "/new", 50),
        ];
        let mut baseline = HashMap::new();
        baseline.insert(PathBuf::from("/a"), 100_u64);
        baseline.insert(PathBuf::from("/b"), 200_u64);

        merge_baseline(&mut ui_nodes, &baseline);

        assert_eq!(ui_nodes[0].prev_size, Some(100));
        assert_eq!(ui_nodes[1].prev_size, Some(200));
        assert_eq!(ui_nodes[2].prev_size, None); // new node, not in baseline
    }

    #[test]
    fn test_merge_baseline_empty_nodes() {
        let mut ui_nodes: Vec<UiNode> = vec![];
        let baseline = HashMap::new();
        merge_baseline(&mut ui_nodes, &baseline);
        assert!(ui_nodes.is_empty());
    }

    #[test]
    fn test_merge_baseline_empty_baseline() {
        let mut ui_nodes = vec![make_ui_node(1, "/a", 100)];
        let baseline = HashMap::new();
        merge_baseline(&mut ui_nodes, &baseline);
        assert_eq!(ui_nodes[0].prev_size, None);
    }
}