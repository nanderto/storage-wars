//! Entry point for the `tree_ops` binary target.
//!
//! This binary is provided for quick smoke-testing and demonstration purposes.
//! The primary API surface is the library crate (`lib.rs`).

use std::collections::HashSet;
use std::path::PathBuf;

use tree_ops::{
    build_baseline_map, build_fs_tree, flatten_tree, insert_children, merge_baseline,
    recalculate_sizes, DbNode, FsNode,
};

fn main() {
    println!("=== tree_ops smoke test ===\n");

    // 1. Build a sample flat DbNode list.
    let db_nodes = vec![
        DbNode {
            id: 1,
            parent_id: None,
            path: PathBuf::from("/data"),
            size: 0,
            child_count: 0,
            is_dir: true,
        },
        DbNode {
            id: 2,
            parent_id: Some(1),
            path: PathBuf::from("/data/docs"),
            size: 0,
            child_count: 0,
            is_dir: true,
        },
        DbNode {
            id: 3,
            parent_id: Some(2),
            path: PathBuf::from("/data/docs/report.pdf"),
            size: 1_024,
            child_count: 0,
            is_dir: false,
        },
        DbNode {
            id: 4,
            parent_id: Some(1),
            path: PathBuf::from("/data/images"),
            size: 0,
            child_count: 0,
            is_dir: true,
        },
        DbNode {
            id: 5,
            parent_id: Some(4),
            path: PathBuf::from("/data/images/photo.jpg"),
            size: 4_096,
            child_count: 0,
            is_dir: false,
        },
        DbNode {
            id: 6,
            parent_id: Some(4),
            path: PathBuf::from("/data/images/thumb.jpg"),
            size: 512,
            child_count: 0,
            is_dir: false,
        },
    ];

    // 2. Build the FsNode hierarchy.
    let mut roots = build_fs_tree(&db_nodes);
    println!("Built tree with {} root(s).", roots.len());

    // 3. Recalculate sizes bottom-up.
    recalculate_sizes(&mut roots);
    println!(
        "Root size after recalculation: {} bytes",
        roots[0].size
    );

    // 4. Insert new children into /data/docs.
    let new_child = FsNode::new(
        7,
        PathBuf::from("/data/docs/notes.txt"),
        256,
        0,
        false,
    );
    let inserted = insert_children(&mut roots, &PathBuf::from("/data/docs"), vec![new_child]);
    println!("insert_children into /data/docs: {}", inserted);

    // 5. Build a baseline map and merge it.
    let baseline_nodes: Vec<FsNode> = db_nodes
        .iter()
        .map(|db| FsNode::new(db.id, db.path.clone(), db.size.saturating_sub(100), 0, db.is_dir))
        .collect();
    let baseline = build_baseline_map(&baseline_nodes);
    merge_baseline(&mut roots, &baseline);
    println!(
        "prev_size of /data/images/photo.jpg: {:?}",
        roots[0]
            .children
            .iter()
            .find(|n| n.path == PathBuf::from("/data/images"))
            .and_then(|n| n.children.iter().find(|c| c.path == PathBuf::from("/data/images/photo.jpg")))
            .and_then(|n| n.prev_size)
    );

    // 6. Flatten the tree for UI rendering.
    let mut expanded = HashSet::new();
    expanded.insert(PathBuf::from("/data"));
    expanded.insert(PathBuf::from("/data/images"));
    let ui_nodes = flatten_tree(&roots, &expanded);
    println!("\nFlattened UI nodes ({} visible):", ui_nodes.len());
    for node in &ui_nodes {
        let indent = "  ".repeat(node.depth);
        println!(
            "{}[{}] {} — size={} progress={:.2}",
            indent,
            if node.is_dir { "D" } else { "F" },
            node.path.display(),
            node.size,
            node.scan_progress,
        );
    }

    println!("\n=== smoke test complete ===");
}