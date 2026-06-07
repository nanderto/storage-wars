//! Finds a parent node by path and replaces its children.

use std::path::PathBuf;
use crate::models::FsNode;

/// Searches the tree rooted at `root` for a node whose `path` matches
/// `parent_path`, then replaces that node's `children` with `new_children`.
///
/// # Arguments
///
/// * `root`        — Mutable reference to the root of the search tree.
/// * `parent_path` — The path of the node whose children should be replaced.
/// * `new_children`— The replacement children list.
///
/// # Returns
///
/// `true` if the parent node was found and updated; `false` otherwise.
pub fn insert_children(
    root: &mut FsNode,
    parent_path: &PathBuf,
    new_children: Vec<FsNode>,
) -> bool {
    if &root.path == parent_path {
        root.children = new_children;
        return true;
    }

    for child in &mut root.children {
        if insert_children(child, parent_path, new_children.clone()) {
            return true;
        }
    }

    false
}

/// Forest variant: searches all roots for the target parent path.
///
/// # Returns
///
/// `true` if the parent node was found and updated in any root; `false` otherwise.
pub fn insert_children_forest(
    roots: &mut Vec<FsNode>,
    parent_path: &PathBuf,
    new_children: Vec<FsNode>,
) -> bool {
    for root in roots.iter_mut() {
        if insert_children(root, parent_path, new_children.clone()) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::models::FsNode;

    fn make_node(id: u64, path: &str, is_dir: bool, children: Vec<FsNode>) -> FsNode {
        FsNode {
            id,
            name: path.split('/').last().unwrap_or(path).into(),
            path: PathBuf::from(path),
            size: 0,
            prev_size: None,
            is_dir,
            file_count: 0,
            children,
        }
    }

    #[test]
    fn test_insert_at_root() {
        let mut root = make_node(1, "/root", true, vec![]);
        let new_child = make_node(2, "/root/child", false, vec![]);
        let found = insert_children(&mut root, &PathBuf::from("/root"), vec![new_child]);
        assert!(found);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].id, 2);
    }

    #[test]
    fn test_insert_at_nested() {
        let mut root = make_node(1, "/root", true, vec![
            make_node(2, "/root/sub", true, vec![]),
        ]);
        let new_child = make_node(3, "/root/sub/file", false, vec![]);
        let found = insert_children(&mut root, &PathBuf::from("/root/sub"), vec![new_child]);
        assert!(found);
        assert_eq!(root.children[0].children.len(), 1);
    }

    #[test]
    fn test_insert_not_found() {
        let mut root = make_node(1, "/root", true, vec![]);
        let found = insert_children(&mut root, &PathBuf::from("/nonexistent"), vec![]);
        assert!(!found);
    }

    #[test]
    fn test_replaces_existing_children() {
        let mut root = make_node(1, "/root", true, vec![
            make_node(2, "/root/old", false, vec![]),
        ]);
        let new_child = make_node(3, "/root/new", false, vec![]);
        insert_children(&mut root, &PathBuf::from("/root"), vec![new_child]);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].id, 3);
    }
}