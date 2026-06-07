//! Bottom-up size and file-count recalculation for `FsNode` trees.

use crate::models::FsNode;

/// Walks the tree bottom-up, recomputing `size` and `file_count` for every
/// directory node from its children.
///
/// Leaf nodes (files) retain their original `size`; their `file_count` is set
/// to `1`. Directory nodes accumulate the `size` and `file_count` of all
/// descendants.
///
/// # Arguments
///
/// * `node` — A mutable reference to the root of the subtree to recalculate.
pub fn recalculate_sizes(node: &mut FsNode) {
    if node.children.is_empty() {
        // Leaf node.
        if !node.is_dir {
            node.file_count = 1;
        }
        return;
    }

    // Recurse into children first (bottom-up).
    for child in &mut node.children {
        recalculate_sizes(child);
    }

    // Aggregate from children.
    node.size = node.children.iter().map(|c| c.size).sum();
    node.file_count = node.children.iter().map(|c| c.file_count).sum();
}

/// Convenience wrapper that recalculates sizes for an entire forest.
pub fn recalculate_sizes_forest(roots: &mut Vec<FsNode>) {
    for root in roots.iter_mut() {
        recalculate_sizes(root);
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
    fn test_leaf_file() {
        let mut node = make_node(1, "file", 512, false, vec![]);
        recalculate_sizes(&mut node);
        assert_eq!(node.size, 512);
        assert_eq!(node.file_count, 1);
    }

    #[test]
    fn test_empty_dir() {
        let mut node = make_node(1, "dir", 0, true, vec![]);
        recalculate_sizes(&mut node);
        assert_eq!(node.size, 0);
        assert_eq!(node.file_count, 0);
    }

    #[test]
    fn test_dir_with_files() {
        let mut root = make_node(1, "root", 0, true, vec![
            make_node(2, "a", 100, false, vec![]),
            make_node(3, "b", 200, false, vec![]),
        ]);
        recalculate_sizes(&mut root);
        assert_eq!(root.size, 300);
        assert_eq!(root.file_count, 2);
    }

    #[test]
    fn test_nested_dirs() {
        let mut root = make_node(1, "root", 0, true, vec![
            make_node(2, "sub", 0, true, vec![
                make_node(3, "file1", 50, false, vec![]),
                make_node(4, "file2", 75, false, vec![]),
            ]),
            make_node(5, "file3", 25, false, vec![]),
        ]);
        recalculate_sizes(&mut root);
        assert_eq!(root.size, 150);
        assert_eq!(root.file_count, 3);

        let sub = &root.children[0];
        assert_eq!(sub.size, 125);
        assert_eq!(sub.file_count, 2);
    }
}