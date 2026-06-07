//! Reconstructs an `FsNode` hierarchy from a flat list of `DbNode` records.

use std::collections::HashMap;
use crate::models::{DbNode, FsNode};

/// Builds a forest of `FsNode` trees from a flat slice of `DbNode` records.
///
/// Nodes are linked via `parent_id`. Nodes whose `parent_id` is `None` or
/// whose `parent_id` does not match any node in the list become roots.
///
/// # Arguments
/// * `nodes` - Flat list of database nodes to reconstruct into a hierarchy.
///
/// # Returns
/// A `Vec<FsNode>` containing the root nodes of the reconstructed forest.
pub fn build_fs_tree(nodes: &[DbNode]) -> Vec<FsNode> {
    if nodes.is_empty() {
        return Vec::new();
    }

    // Build a set of all known IDs for quick parent validation.
    let known_ids: std::collections::HashSet<u64> = nodes.iter().map(|n| n.id).collect();

    // Map from id → FsNode (children populated later).
    let mut node_map: HashMap<u64, FsNode> = nodes
        .iter()
        .map(|db| {
            (
                db.id,
                FsNode {
                    id: db.id,
                    path: db.path.clone(),
                    size: db.size,
                    item_count: db.item_count,
                    is_dir: db.is_dir,
                    children: Vec::new(),
                },
            )
        })
        .collect();

    // Determine which nodes are roots (no parent_id or parent not in set).
    let root_ids: Vec<u64> = nodes
        .iter()
        .filter(|n| n.parent_id.map_or(true, |pid| !known_ids.contains(&pid)))
        .map(|n| n.id)
        .collect();

    // Build parent → children mapping.
    let mut children_map: HashMap<u64, Vec<u64>> = HashMap::new();
    for node in nodes {
        if let Some(pid) = node.parent_id {
            if known_ids.contains(&pid) {
                children_map.entry(pid).or_default().push(node.id);
            }
        }
    }

    // Recursively attach children using an iterative post-order approach.
    fn attach_children(
        id: u64,
        node_map: &mut HashMap<u64, FsNode>,
        children_map: &HashMap<u64, Vec<u64>>,
    ) -> FsNode {
        let child_ids = children_map.get(&id).cloned().unwrap_or_default();
        let children: Vec<FsNode> = child_ids
            .into_iter()
            .map(|cid| attach_children(cid, node_map, children_map))
            .collect();

        let mut node = node_map.remove(&id).expect("node must exist in map");
        node.children = children;
        node
    }

    root_ids
        .into_iter()
        .map(|rid| attach_children(rid, &mut node_map, &children_map))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_db_node(id: u64, parent_id: Option<u64>, path: &str, size: u64) -> DbNode {
        DbNode {
            id,
            parent_id,
            path: PathBuf::from(path),
            size,
            item_count: 0,
            is_dir: parent_id.is_none(),
        }
    }

    #[test]
    fn test_empty_input() {
        let result = build_fs_tree(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_root() {
        let nodes = vec![make_db_node(1, None, "/root", 100)];
        let tree = build_fs_tree(&nodes);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].id, 1);
        assert!(tree[0].children.is_empty());
    }

    #[test]
    fn test_parent_child_relationship() {
        let nodes = vec![
            make_db_node(1, None, "/root", 200),
            make_db_node(2, Some(1), "/root/a", 100),
            make_db_node(3, Some(1), "/root/b", 100),
        ];
        let tree = build_fs_tree(&nodes);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].children.len(), 2);
    }

    #[test]
    fn test_orphan_becomes_root() {
        let nodes = vec![
            make_db_node(1, None, "/root", 100),
            make_db_node(2, Some(999), "/orphan", 50),
        ];
        let tree = build_fs_tree(&nodes);
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_deep_nesting() {
        let nodes = vec![
            make_db_node(1, None, "/a", 300),
            make_db_node(2, Some(1), "/a/b", 200),
            make_db_node(3, Some(2), "/a/b/c", 100),
        ];
        let tree = build_fs_tree(&nodes);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].children.len(), 1);
        assert_eq!(tree[0].children[0].children[0].id, 3);
    }
}