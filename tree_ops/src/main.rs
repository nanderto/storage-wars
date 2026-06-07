//! Entry point for the tree_ops binary.
//!
//! This binary serves as a demonstration and integration test harness for the
//! tree_ops library. In production, tree_ops is consumed as a library crate.

use std::collections::HashSet;
use std::path::PathBuf;
use tree_ops::{
    build_baseline_map, build_fs_tree, flatten_tree, insert_children, merge_baseline,
    recalculate_sizes, DbNode, FsNode,
};

fn main() {
    println!("tree_ops — Tree manipulation utilities");
    println!("======================================");

    // ── 1. Build a tree from flat DbNode records ──────────────────────────────
    let db_nodes = vec![
        DbNode {
            id: 1,
            parent_id: None,
            name: "home".to_string(),
            path: PathBuf::from("/home"),
            size: 0,
            item_count: 0,
            is_dir: true,
        },
        DbNode {
            id: 2,
            parent_id: Some(1),
            name: "documents".to_string(),
            path: PathBuf::from("/home/documents"),
            size: 0,
            item_count: 0,
            is_dir: true,
        },
        DbNode {
            id: 3,
            parent_id: Some(2),
            name: "report.pdf".to_string(),
            path: PathBuf::from("/home/documents/report.pdf"),
            size: 204_800,
            item_count: 0,
            is_dir: false,
        },
        DbNode {
            id: 4,
            parent_id: Some(1),
            name: "photo.jpg".to_string(),
            path: PathBuf::from("/home/photo.jpg"),
            size: 3_145_728,
            item_count: 0,
            is_dir: false,
        },
    ];

    let mut tree = build_fs_tree(db_nodes);
    println!("\n[1] Built tree from {} DbNode records.", 4);
    println!("    Root nodes: {}", tree.len());

    // ── 2. Recalculate sizes bottom-up ────────────────────────────────────────
    recalculate_sizes(&mut tree);
    println!("\n[2] Recalculated sizes.");
    println!("    Root size: {} bytes", tree[0].size);
    println!("    Root item_count: {}", tree[0].item_count);

    // ── 3. Build a baseline map ───────────────────────────────────────────────
    let baseline = build_baseline_map(&tree);
    println!("\n[3] Built baseline map with {} entries.", baseline.len());

    // ── 4. Insert new children into a node ───────────────────────────────────
    let new_file = FsNode {
        id: 5,
        parent_id: Some(2),
        name: "notes.txt".to_string(),
        path: PathBuf::from("/home/documents/notes.txt"),
        size: 1_024,
        item_count: 0,
        is_dir: false,
        children: Vec::new(),
        prev_size: None,
    };
    let inserted = insert_children(
        &mut tree,
        &PathBuf::from("/home/documents"),
        vec![new_file],
    );
    println!("\n[4] insert_children into /home/documents: {}", inserted);

    // Recalculate after insertion.
    recalculate_sizes(&mut tree);
    println!("    Root size after insertion: {} bytes", tree[0].size);

    // ── 5. Merge baseline (populate prev_size) ────────────────────────────────
    merge_baseline(&mut tree, &baseline);
    println!("\n[5] Merged baseline into tree.");
    println!(
        "    Root prev_size: {:?}",
        tree[0].prev_size
    );

    // ── 6. Flatten tree for UI rendering ─────────────────────────────────────
    let mut expanded = HashSet::new();
    expanded.insert(PathBuf::from("/home"));
    expanded.insert(PathBuf::from("/home/documents"));

    let ui_nodes = flatten_tree(&tree, &expanded);
    println!("\n[6] Flattened tree into {} UiNode(s):", ui_nodes.len());
    for node in &ui_nodes {
        let indent = "  ".repeat(node.depth);
        println!(
            "    {}{} (size={}, progress={:.2}, prev={:?})",
            indent, node.name, node.size, node.scan_progress, node.prev_size
        );
    }

    println!("\nAll operations completed successfully.");
}