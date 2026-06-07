// path: tree_ops/src/main.rs
//! Entry point for the tree_ops binary (demonstration / smoke test).
//!
//! The primary interface is the library crate (`lib.rs`). This binary
//! provides a minimal runnable entry point so `cargo run` succeeds.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use tree_ops::{
    build_baseline_map, build_fs_tree, flatten_tree, insert_children, merge_baseline,
    recalculate_sizes, DbNode, FsNode,
};

fn main() {
    println!("tree_ops — tree manipulation utilities");

    // ── 1. Build a sample flat DbNode list ──────────────────────────────────
    let db_nodes: Vec<DbNode> = vec![
        DbNode {
            id: 1,
            parent_id: None,
            path: PathBuf::from("/home/user"),
            size: 0,
            item_count: 0,
            is_dir: true,
        },
        DbNode {
            id: 2,
            parent_id: Some(1),
            path: PathBuf::from("/home/user/documents"),
            size: 0,
            item_count: 0,
            is_dir: true,
        },
        DbNode {
            id: 3,
            parent_id: Some(2),
            path: PathBuf::from("/home/user/documents/report.pdf"),
            size: 2_048_000,
            item_count: 0,
            is_dir: false,
        },
        DbNode {
            id: 4,
            parent_id: Some(1),
            path: PathBuf::from("/home/user/pictures"),
            size: 0,
            item_count: 0,
            is_dir: true,
        },
        DbNode {
            id: 5,
            parent_id: Some(4),
            path: PathBuf::from("/home/user/pictures/photo.jpg"),
            size: 5_120_000,
            item_count: 0,
            is_dir: false,
        },
    ];

    // ── 2. Reconstruct the FsNode hierarchy ─────────────────────────────────
    let mut forest: Vec<FsNode> = build_fs_tree(&db_nodes);
    println!("\n[build_fs_tree] Root nodes: {}", forest.len());

    // ── 3. Recalculate sizes bottom-up ──────────────────────────────────────
    recalculate_sizes(&mut forest);
    if let Some(root) = forest.first() {
        println!(
            "[recalculate_sizes] Root '{}' size={} item_count={}",
            root.path.display(),
            root.size,
            root.item_count
        );
    }

    // ── 4. Build a baseline map (snapshot before changes) ───────────────────
    let baseline: HashMap<PathBuf, u64> = build_baseline_map(&forest);
    println!("[build_baseline_map] Entries: {}", baseline.len());

    // ── 5. Insert new children into a node ──────────────────────────────────
    let new_children = vec![FsNode {
        id: 6,
        path: PathBuf::from("/home/user/documents/notes.txt"),
        size: 4_096,
        item_count: 0,
        is_dir: false,
        children: vec![],
    }];
    let updated = insert_children(
        &mut forest,
        &PathBuf::from("/home/user/documents"),
        new_children,
    );
    println!("[insert_children] Parent found and updated: {updated}");

    // Recalculate after structural change.
    recalculate_sizes(&mut forest);

    // ── 6. Flatten the tree for UI rendering ────────────────────────────────
    let mut expanded: HashSet<PathBuf> = HashSet::new();
    expanded.insert(PathBuf::from("/home/user"));
    expanded.insert(PathBuf::from("/home/user/documents"));
    expanded.insert(PathBuf::from("/home/user/pictures"));

    let mut ui_nodes = flatten_tree(&forest, &expanded);
    println!("[flatten_tree] UI nodes: {}", ui_nodes.len());

    // ── 7. Merge baseline into UI nodes for change detection ─────────────────
    merge_baseline(&mut ui_nodes, &baseline);
    println!("[merge_baseline] Nodes with prev_size:");
    for n in &ui_nodes {
        if let Some(prev) = n.prev_size {
            println!(
                "  {} size={} prev_size={} delta={}",
                n.path.display(),
                n.size,
                prev,
                n.size as i64 - prev as i64
            );
        }
    }

    println!("\nAll operations completed successfully.");
}