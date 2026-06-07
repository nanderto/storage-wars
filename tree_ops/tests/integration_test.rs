//! Integration tests for tree_ops end-to-end workflows.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tree_ops::{
    build_baseline_map, build_fs_tree, flatten_tree, insert_children, merge_baseline,
    recalculate_sizes, DbNode, FsNode,
};

fn db(id: u64, parent_id: Option<u64>, path: &str, size: u64, is_dir: bool) -> DbNode {
    DbNode {
        id,
        parent_id,
        path: PathBuf::from(path),
        size,
        file_count: if is_dir { 0 } else { 1 },
        is_dir,
    }
}

#[test]
fn test_full_pipeline() {
    // 1. Build tree from flat DbNode list.
    let db_nodes = vec![
        db(1, None, "/home", 0, true),
        db(2, Some(1), "/home/user", 0, true),
        db(3, Some(2), "/home/user/doc.txt", 500, false),
        db(4, Some(2), "/home/user/img.png", 1500, false),
    ];

    let mut roots = build_fs_tree(&db_nodes);
    assert_eq!(roots.len(), 1);

    // 2. Recalculate sizes bottom-up.
    recalculate_sizes(&mut roots[0]);
    assert_eq!(roots[0].size, 2000);
    assert_eq!(roots[0].file_count, 2);

    // 3. Build baseline map before modification.
    let baseline = build_baseline_map(&roots);

    // 4. Insert new children into /home/user.
    let new_child = FsNode {
        id: 5,
        path: PathBuf::from("/home/user/new.txt"),
        size: 300,
        file_count: 1,
        is_dir: false,
        children: vec![],
        scan_progress: None,
    };
    let found = insert_children(&mut roots[0], &PathBuf::from("/home/user"), vec![new_child]);
    assert!(found);

    // 5. Recalculate after insertion.
    recalculate_sizes(&mut roots[0]);
    assert_eq!(roots[0].size, 300);

    // 6. Flatten tree with expansion.
    let mut expanded = HashSet::new();
    expanded.insert(PathBuf::from("/home"));
    expanded.insert(PathBuf::from("/home/user"));

    let mut ui_nodes = flatten_tree(&roots[0], &expanded, 0);
    assert_eq!(ui_nodes.len(), 3); // /home, /home/user, /home/user/new.txt

    // 7. Merge baseline to populate prev_size.
    merge_baseline(&mut ui_nodes, &baseline);

    let home_node = ui_nodes.iter().find(|n| n.path == PathBuf::from("/home")).unwrap();
    assert_eq!(home_node.prev_size, Some(2000));
}

#[test]
fn test_build_baseline_map_empty() {
    let map: HashMap<PathBuf, u64> = build_baseline_map(&[]);
    assert!(map.is_empty());
}

#[test]
fn test_flatten_collapsed_hides_children() {
    let db_nodes = vec![
        db(1, None, "/root", 0, true),
        db(2, Some(1), "/root/a", 100, false),
        db(3, Some(1), "/root/b", 200, false),
    ];
    let roots = build_fs_tree(&db_nodes);
    // No paths expanded → only root visible.
    let ui_nodes = flatten_tree(&roots[0], &HashSet::new(), 0);
    assert_eq!(ui_nodes.len(), 1);
}