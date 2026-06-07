//! Binary entry point for the `tree_ops` crate.
//!
//! This binary is intentionally minimal. All production logic lives in the
//! library (`lib.rs`). The binary exists so the crate can be compiled as both
//! a library and a standalone executable for quick smoke-testing.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use tree_ops::{
    build_baseline_map, build_fs_tree, flatten_tree, insert_children,
    merge_baseline, recalculate_sizes, DbNode, FsNode,
};
use tree_ops::recalculate_sizes::recalculate_sizes_forest;
use tree_ops::baseline::merge_baseline_forest;

fn main() {
    println!("tree_ops — smoke test");

    // ── 1. Build a tree from flat DbNodes ────────────────────────────────────
    let db_nodes = vec![
        DbNode { id: 1, parent_id: None,    name: "root".into(),  path: PathBuf::from("/root"),       size: 0,    is_dir: true,  child_count: 2 },
        DbNode { id: 2, parent_id: Some(1), name: "docs".into(),  path: PathBuf::from("/root/docs"),  size: 0,    is_dir: true,  child_count: 1 },
        DbNode { id: 3, parent_id: Some(2), name: "a.txt".into(), path: PathBuf::from("/root/docs/a.txt"), size: 1024, is_dir: false, child_count: 0 },
        DbNode { id: 4, parent_id: Some(1), name: "b.bin".into(), path: PathBuf::from("/root/b.bin"), size: 2048, is_dir: false, child_count: 0 },
    ];

    let mut forest = build_fs_tree(&db_nodes);
    println!("Roots after build_fs_tree: {}", forest.len());

    // ── 2. Recalculate sizes bottom-up ───────────────────────────────────────
    recalculate_sizes_forest(&mut forest);
    println!("Root size after recalculate: {}", forest[0].size);

    // ── 3. Build baseline snapshot ───────────────────────────────────────────
    let baseline = build_baseline_map(&forest);
    println!("Baseline entries: {}", baseline.len());

    // ── 4. Simulate a size change and merge baseline ──────────────────────────
    if let Some(root) = forest.first_mut() {
        if let Some(child) = root.children.iter_mut().find(|c| c.name == "b.bin") {
            child.size = 4096;
        }
    }
    merge_baseline_forest(&mut forest, &baseline);
    println!(
        "b.bin prev_size: {:?}",
        forest[0].children.iter().find(|c| c.name == "b.bin").and_then(|n| n.prev_size)
    );

    // ── 5. Insert children ────────────────────────────────────────────────────
    let new_child = FsNode::new(5, "c.txt", PathBuf::from("/root/docs/c.txt"), 512, false);
    if let Some(root) = forest.first_mut() {
        insert_children(root, &PathBuf::from("/root/docs"), vec![new_child]);
    }
    println!(
        "docs children after insert: {}",
        forest[0]
            .children
            .iter()
            .find(|c| c.name == "docs")
            .map(|d| d.children.len())
            .unwrap_or(0)
    );

    // ── 6. Flatten for UI ─────────────────────────────────────────────────────
    let mut expanded = HashSet::new();
    expanded.insert(PathBuf::from("/root"));
    expanded.insert(PathBuf::from("/root/docs"));

    let ui_nodes = flatten_tree(&forest, &expanded);
    println!("UI nodes visible: {}", ui_nodes.len());
    for ui in &ui_nodes {
        println!(
            "  {:>depth$}{name}  size={size}  progress={progress:.2}",
            "",
            depth = ui.depth * 2,
            name = ui.name,
            size = ui.size,
            progress = ui.scan_progress,
        );
    }

    println!("smoke test passed ✓");
}