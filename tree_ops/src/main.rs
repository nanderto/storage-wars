//! Entry point for the `tree_ops` binary.
//!
//! This binary serves as a demonstration and integration harness for the
//! tree_ops library. In production, the library crate is consumed directly
//! by the desktop application.

use std::collections::HashSet;
use std::path::PathBuf;

use tree_ops::{
    build_baseline_map, build_fs_tree, flatten_tree, insert_children, merge_baseline,
    recalculate_sizes, DbNode, FsNode,
};

fn main() {
    println!("tree_ops — filesystem tree manipulation utilities");
    println!("=================================================");

    // ── 1. Build a tree from flat DbNode records ──────────────────────────
    let db_nodes = vec![
        DbNode {
            id: 1,
            parent_id: None,
            path: PathBuf::from("/home/user"),
            size: 0,
            is_dir: true,
            child_count: 2,
            scan_progress: None,
        },
        DbNode {
            id: 2,
            parent_id: Some(1),
            path: PathBuf::from("/home/user/documents"),
            size: 0,
            is_dir: true,
            child_count: 1,
            scan_progress: None,
        },
        DbNode {
            id: 3,
            parent_id: Some(2),
            path: PathBuf::from("/home/user/documents/report.pdf"),
            size: 2_048_000,
            is_dir: false,
            child_count: 0,
            scan_progress: None,
        },
        DbNode {
            id: 4,
            parent_id: Some(1),
            path: PathBuf::from("/home/user/pictures"),
            size: 0,
            is_dir: true,
            child_count: 1,
            scan_progress: None,
        },
        DbNode {
            id: 5,
            parent_id: Some(4),
            path: PathBuf::from("/home/user/pictures/photo.jpg"),
            size: 5_120_000,
            is_dir: false,
            child_count: 0,
            scan_progress: None,
        },
    ];

    let mut roots = build_fs_tree(db_nodes);
    println!("\n[1] build_fs_tree: {} root(s) constructed", roots.len());

    // ── 2. Recalculate sizes bottom-up ────────────────────────────────────
    recalculate_sizes(&mut roots);
    println!(
        "[2] recalculate_sizes: root size = {} bytes, file_count = {}",
        roots[0].size, roots[0].file_count
    );

    // ── 3. Build a baseline snapshot ─────────────────────────────────────
    let baseline = build_baseline_map(&roots);
    println!("[3] build_baseline_map: {} entries in baseline", baseline.len());

    // ── 4. Simulate a new scan and merge baseline ─────────────────────────
    let mut new_roots = roots.clone();
    // Simulate a new file appearing and sizes changing.
    let new_file = FsNode::new(
        6,
        PathBuf::from("/home/user/documents/notes.txt"),
        4_096,
        1,
        false,
    );
    insert_children(
        &mut new_roots,
        PathBuf::from("/home/user/documents").as_path(),
        vec![
            FsNode::new(
                3,
                PathBuf::from("/home/user/documents/report.pdf"),
                2_048_000,
                1,
                false,
            ),
            new_file,
        ],
    );
    recalculate_sizes(&mut new_roots);
    merge_baseline(&mut new_roots, &baseline);
    println!(
        "[4] merge_baseline: root prev_size = {:?}",
        new_roots[0].prev_size
    );

    // ── 5. Flatten for UI rendering ───────────────────────────────────────
    let mut expanded = HashSet::new();
    expanded.insert(PathBuf::from("/home/user"));
    expanded.insert(PathBuf::from("/home/user/documents"));

    let ui_nodes = flatten_tree(&new_roots, &expanded);
    println!("[5] flatten_tree: {} UI nodes visible", ui_nodes.len());
    for ui_node in &ui_nodes {
        let indent = "  ".repeat(ui_node.depth);
        let fraction = ui_node
            .size_fraction
            .map(|f| format!("{:.1}%", f * 100.0))
            .unwrap_or_else(|| "n/a".to_string());
        println!(
            "{}[{}] {} — {} bytes ({})",
            indent,
            if ui_node.is_dir { "DIR" } else { "FILE" },
            ui_node.path.display(),
            ui_node.size,
            fraction,
        );
    }

    println!("\nAll operations completed successfully.");
}