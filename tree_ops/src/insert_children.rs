//! Finds a parent `FsNode` by path and replaces its children.

use std::path::Path;
use crate::models::FsNode;

/// Searches the forest for a node matching `parent_path` and replaces its
/// `children` with the provided `new_children`.
///
/// The search is performed depth-first (pre-order). Returns `true` if the
/// parent was found and updated, `false` otherwise.
///
/// # Arguments
/// * `nodes` - Mutable reference to the forest of root nodes.
/// * `parent_path` - The path of the node whose children should be replaced.
/// * `new_children` - The new children to assign to the matched node.
pub fn insert_children(
    nodes: &mut Vec<FsNode>,
    parent_path: &Path,
    new_children: Vec<FsNode>,
) -> bool {
    for node in nodes.iter_mut() {
        if node.path == parent_path {
            node.children = new_children;
            return true;
        }
        if insert_children(&mut node.children, parent_path, new_children.clone()) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_node(id: u64, path: &str, children: Vec<FsNode>) -> FsNode {
        FsNode {
            id,
            path: PathBuf::from(path),
            size: 0,
            item_count: 0,
            is_dir: !children.is_empty(),
            children,
        }
    }

    #[test]
    fn test_insert_at_root() {
        let mut forest = vec![make_node(1, "/root", vec![])];
        let new_children = vec![make_node(2, "/root/a", vec![])];
        let found = insert_children(&mut forest, Path::new("/root"), new_children);
        assert!(found);
        assert_eq!(forest[0].children.len(), 1);
        assert_eq!(forest[0].children[0].id, 2);
    }

    #[test]
    fn test_insert_at_nested_node() {
        let mut forest = vec![make_node(1, "/root", vec![
            make_node(2, "/root/sub", vec![]),
        ])];
        let new_children = vec![make_node(3, "/root/sub/file.txt", vec![])];
        let found = insert_children(&mut forest, Path::new("/root/sub"), new_children);
        assert!(found);
        assert_eq!(forest[0].children[0].children.len(), 1);
    }

    #[test]
    fn test_not_found_returns_false() {
        let mut forest = vec![make_node(1, "/root", vec![])];
        let found = insert_children(&mut forest, Path::new("/nonexistent"), vec![]);
        assert!(!found);
    }

    #[test]
    fn test_replaces_existing_children() {
        let mut forest = vec![make_node(1, "/root", vec![
            make_node(2, "/root/old", vec![]),
        ])];
        let new_children = vec![make_node(3, "/root/new", vec![])];
        insert_children(&mut forest, Path::new("/root"), new_children);
        assert_eq!(forest[0].children.len(), 1);
        assert_eq!(forest[0].children[0].id, 3);
    }

    #[test]
    fn test_empty_forest() {
        let mut forest: Vec<FsNode> = vec![];
        let found = insert_children(&mut forest, Path::new("/root"), vec![]);
        assert!(!found);
    }
}