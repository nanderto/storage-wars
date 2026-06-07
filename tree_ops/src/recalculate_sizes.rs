//! Bottom-up size and item count recalculation for `FsNode` trees.

use crate::models::FsNode;

/// Recalculates `size` and `item_count` for every node in the forest by
/// walking the tree bottom-up (post-order).
///
/// For each directory node:
/// - `size` is set to the sum of all children's sizes.
/// - `item_count` is set to the total number of descendant nodes (not just direct children).
///
/// Leaf nodes (files) retain their original `size` and get `item_count = 0`.
///
/// # Arguments
/// * `nodes` - Mutable slice of root `FsNode` trees to recalculate.
pub fn recalculate_sizes(nodes: &mut Vec<FsNode>) {
    for node in nodes.iter_mut() {
        recalculate_node(node);
    }
}

/// Recursively recalculates size and item_count for a single node.
/// Returns `(size, item_count)` for the node after recalculation.
fn recalculate_node(node: &mut FsNode) -> (u64, u64) {
    if node.children.is_empty() {
        // Leaf node: keep its own size, item_count = 0.
        node.item_count = 0;
        return (node.size, 0);
    }

    let mut total_size: u64 = 0;
    let mut total_count: u64 = 0;

    for child in node.children.iter_mut() {
        let (child_size, child_count) = recalculate_node(child);
        total_size = total_size.saturating_add(child_size);
        // Each child counts as 1 item, plus all of its descendants.
        total_count = total_count.saturating_add(1).saturating_add(child_count);
    }

    node.size = total_size;
    node.item_count = total_count;

    (total_size, total_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_leaf(id: u64, path: &str, size: u64) -> FsNode {
        FsNode {
            id,
            path: PathBuf::from(path),
            size,
            item_count: 0,
            is_dir: false,
            children: vec![],
        }
    }

    fn make_dir(id: u64, path: &str, children: Vec<FsNode>) -> FsNode {
        FsNode {
            id,
            path: PathBuf::from(path),
            size: 0,
            item_count: 0,
            is_dir: true,
            children,
        }
    }

    #[test]
    fn test_leaf_unchanged() {
        let mut nodes = vec![make_leaf(1, "/file.txt", 500)];
        recalculate_sizes(&mut nodes);
        assert_eq!(nodes[0].size, 500);
        assert_eq!(nodes[0].item_count, 0);
    }

    #[test]
    fn test_single_level_dir() {
        let mut nodes = vec![make_dir(1, "/root", vec![
            make_leaf(2, "/root/a.txt", 100),
            make_leaf(3, "/root/b.txt", 200),
        ])];
        recalculate_sizes(&mut nodes);
        assert_eq!(nodes[0].size, 300);
        assert_eq!(nodes[0].item_count, 2);
    }

    #[test]
    fn test_nested_dirs() {
        let mut nodes = vec![make_dir(1, "/root", vec![
            make_dir(2, "/root/sub", vec![
                make_leaf(3, "/root/sub/file.txt", 400),
            ]),
            make_leaf(4, "/root/other.txt", 100),
        ])];
        recalculate_sizes(&mut nodes);
        // /root/sub: size=400, item_count=1
        assert_eq!(nodes[0].children[0].size, 400);
        assert_eq!(nodes[0].children[0].item_count, 1);
        // /root: size=500, item_count=3 (sub + file inside sub + other)
        assert_eq!(nodes[0].size, 500);
        assert_eq!(nodes[0].item_count, 3);
    }

    #[test]
    fn test_empty_forest() {
        let mut nodes: Vec<FsNode> = vec![];
        recalculate_sizes(&mut nodes);
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_saturating_add_does_not_panic() {
        let mut nodes = vec![make_dir(1, "/root", vec![
            make_leaf(2, "/root/a", u64::MAX / 2),
            make_leaf(3, "/root/b", u64::MAX / 2 + 1),
        ])];
        // Should not panic; saturating_add prevents overflow.
        recalculate_sizes(&mut nodes);
        assert_eq!(nodes[0].size, u64::MAX);
    }
}