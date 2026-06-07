//! Baseline snapshot utilities: building a path→size map and merging it into a tree.

use std::collections::HashMap;
use std::path::PathBuf;
use crate::models::{BaselineMap, FsNode};

/// Builds a `BaselineMap` (`PathBuf → u64`) from a slice of `FsNode` trees.
///
/// Every node in the forest (including all descendants) is visited and its
/// `path → size` pair is inserted into the map.
///
/// # Arguments
///
/// * `roots` — Root nodes of the forest to snapshot.
///
/// # Returns
///
/// A `HashMap<PathBuf, u64>` mapping each node's path to its current size.
pub fn build_baseline_map(roots: &[FsNode]) -> BaselineMap {
    let mut map = HashMap::new();
    for root in roots {
        collect_into_map(root, &mut map);
    }
    map
}

fn collect_into_map(node: &FsNode, map: &mut BaselineMap) {
    map.insert(node.path.clone(), node.size);
    for child in &node.children {
        collect_into_map(child, map);
    }
}

/// Populates the `prev_size` field of every node in the tree from a baseline map.
///
/// If a node's path is present in `baseline`, `prev_size` is set to the
/// corresponding value. Otherwise `prev_size` is left as `None`.
///
/// # Arguments
///
/// * `node`     — Mutable reference to the root of the subtree to update.
/// * `baseline` — The baseline map produced by [`build_baseline_map`].
pub fn merge_baseline(node: &mut FsNode, baseline: &BaselineMap) {
    node.prev_size = baseline.get(&node.path).copied();
    for child in &mut node.children {
        merge_baseline(child, baseline);
    }
}

/// Forest variant of [`merge_baseline`].
pub fn merge_baseline_forest(roots: &mut Vec<FsNode>, baseline: &BaselineMap) {
    for root in roots.iter_mut() {
        merge_baseline(root, baseline);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::models::FsNode;

    fn make_node(id: u64, path: &str, size: u64, children: Vec<FsNode>) -> FsNode {
        FsNode {
            id,
            name: path.split('/').last().unwrap_or(path).into(),
            path: PathBuf::from(path),
            size,
            prev_size: None,
            is_dir: !children.is_empty(),
            file_count: 0,
            children,
        }
    }

    #[test]
    fn test_build_baseline_map_empty() {
        let map = build_baseline_map(&[]);
        assert!(map.is_empty());
    }

    #[test]
    fn test_build_baseline_map_single() {
        let node = make_node(1, "/root", 1024, vec![]);
        let map = build_baseline_map(&[node]);
        assert_eq!(map.get(&PathBuf::from("/root")), Some(&1024));
    }

    #[test]
    fn test_build_baseline_map_nested() {
        let root = make_node(1, "/root", 300, vec![
            make_node(2, "/root/a", 100, vec![]),
            make_node(3, "/root/b", 200, vec![]),
        ]);
        let map = build_baseline_map(&[root]);
        assert_eq!(map.len(), 3);
        assert_eq!(map[&PathBuf::from("/root")], 300);
        assert_eq!(map[&PathBuf::from("/root/a")], 100);
        assert_eq!(map[&PathBuf::from("/root/b")], 200);
    }

    #[test]
    fn test_merge_baseline_sets_prev_size() {
        let mut node = make_node(1, "/root", 500, vec![]);
        let mut baseline = HashMap::new();
        baseline.insert(PathBuf::from("/root"), 400u64);
        merge_baseline(&mut node, &baseline);
        assert_eq!(node.prev_size, Some(400));
    }

    #[test]
    fn test_merge_baseline_missing_path() {
        let mut node = make_node(1, "/root", 500, vec![]);
        let baseline = HashMap::new();
        merge_baseline(&mut node, &baseline);
        assert_eq!(node.prev_size, None);
    }

    #[test]
    fn test_merge_baseline_recursive() {
        let mut root = make_node(1, "/root", 300, vec![
            make_node(2, "/root/child", 100, vec![]),
        ]);
        let mut baseline = HashMap::new();
        baseline.insert(PathBuf::from("/root"), 250u64);
        baseline.insert(PathBuf::from("/root/child"), 80u64);
        merge_baseline(&mut root, &baseline);
        assert_eq!(root.prev_size, Some(250));
        assert_eq!(root.children[0].prev_size, Some(80));
    }

    #[test]
    fn test_round_trip_baseline() {
        let roots = vec![make_node(1, "/root", 1000, vec![
            make_node(2, "/root/x", 600, vec![]),
            make_node(3, "/root/y", 400, vec![]),
        ])];
        let baseline = build_baseline_map(&roots);

        let mut updated_roots = vec![make_node(1, "/root", 1200, vec![
            make_node(2, "/root/x", 700, vec![]),
            make_node(3, "/root/y", 500, vec![]),
        ])];
        merge_baseline_forest(&mut updated_roots, &baseline);

        assert_eq!(updated_roots[0].prev_size, Some(1000));
        assert_eq!(updated_roots[0].children[0].prev_size, Some(600));
        assert_eq!(updated_roots[0].children[1].prev_size, Some(400));
    }
}