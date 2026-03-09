use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{DbNode, FsNode, UiNode};

/// Format a `SystemTime` as an ISO 8601 string (UTC, second precision).
fn format_system_time(t: std::time::SystemTime) -> String {
    let secs = t
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let sec = secs % 60;
    let min = (secs / 60) % 60;
    let hour = (secs / 3600) % 24;
    let days = secs / 86400;
    let (year, month, day) = crate::persistence::days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}Z")
}

// ---------------------------------------------------------------------------
// Synchronous directory scanner (run on a background thread)
// ---------------------------------------------------------------------------

/// Recursively scan `root`, summing child sizes bottom-up.
/// Permission errors are silently skipped (size treated as 0).
pub fn scan_dir_sync(root: &Path) -> FsNode {
    let meta = fs::metadata(root);
    let name = root
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| root.to_string_lossy().into_owned());

    let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);

    let modified = meta
        .as_ref()
        .ok()
        .and_then(|m| m.modified().ok())
        .map(format_system_time);

    if !is_dir {
        let size = meta.map(|m| m.len()).unwrap_or(0);
        return FsNode {
            name,
            path: root.to_path_buf(),
            is_dir: false,
            current_size: size,
            prev_size: None,
            children: vec![],
            file_count: 0,
            folder_count: 0,
            modified,
        };
    }

    let mut children = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            children.push(scan_dir_sync(&entry.path()));
        }
    }

    let total: u64 = children.iter().map(|c| c.current_size).sum();
    let file_count: u64 = children.iter().map(|c| {
        if c.is_dir { c.file_count } else { 1 }
    }).sum();
    let folder_count: u64 = children.iter().map(|c| {
        if c.is_dir { 1 + c.folder_count } else { 0 }
    }).sum();

    FsNode {
        name,
        path: root.to_path_buf(),
        is_dir: true,
        current_size: total,
        prev_size: None,
        children,
        file_count,
        folder_count,
        modified,
    }
}

// ---------------------------------------------------------------------------
// Tree flattening — turns the nested FsNode tree into a flat list for the UI
// ---------------------------------------------------------------------------

/// Flatten `roots` into a `Vec<UiNode>` for rendering.
/// Only expands nodes whose path is in `expanded_paths`.
/// `scan_progress` for each node is its fraction of the largest sibling's size.
pub fn flatten_tree(roots: &[FsNode], expanded_paths: &HashSet<PathBuf>) -> Vec<UiNode> {
    let max_size = roots.iter().map(|n| n.current_size).max().unwrap_or(0);
    let mut out = Vec::new();
    for node in roots {
        flatten_node(node, 0, max_size, expanded_paths, &mut out);
    }
    out
}

fn flatten_node(
    node: &FsNode,
    depth: usize,
    parent_max_size: u64,
    expanded_paths: &HashSet<PathBuf>,
    out: &mut Vec<UiNode>,
) {
    let scan_progress = if parent_max_size == 0 {
        0.0_f32
    } else {
        (node.current_size as f64 / parent_max_size as f64).min(1.0) as f32
    };

    let expanded = node.is_dir && expanded_paths.contains(&node.path);

    out.push(UiNode {
        fs_node: node.clone(),
        depth,
        expanded,
        scan_progress,
    });

    if expanded {
        let child_max = node.children.iter().map(|c| c.current_size).max().unwrap_or(0);
        for child in &node.children {
            flatten_node(child, depth + 1, child_max, expanded_paths, out);
        }
    }
}

// ---------------------------------------------------------------------------
// Baseline helpers — populate `prev_size` from a prior scan for comparison
// ---------------------------------------------------------------------------

/// Build a path → size lookup from a flat slice of `DbNode`s.
pub fn build_baseline_map(nodes: &[DbNode]) -> HashMap<PathBuf, u64> {
    nodes.iter().map(|n| (PathBuf::from(&n.path), n.size)).collect()
}

/// Walk `nodes` recursively, setting `prev_size` from `baseline` where a path matches.
pub fn merge_baseline(nodes: &mut [FsNode], baseline: &HashMap<PathBuf, u64>) {
    for node in nodes.iter_mut() {
        node.prev_size = baseline.get(&node.path).copied();
        merge_baseline(&mut node.children, baseline);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DbNode;
    use std::path::PathBuf;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_tree() -> Vec<FsNode> {
        vec![FsNode {
            name: "root".into(),
            path: PathBuf::from("/root"),
            is_dir: true,
            current_size: 300,
            prev_size: None,
            children: vec![
                FsNode {
                    name: "docs".into(),
                    path: PathBuf::from("/root/docs"),
                    is_dir: true,
                    current_size: 200,
                    prev_size: None,
                    children: vec![FsNode {
                        name: "readme.txt".into(),
                        path: PathBuf::from("/root/docs/readme.txt"),
                        is_dir: false,
                        current_size: 200,
                        prev_size: None,
                        children: vec![],
                        file_count: 0,
                        folder_count: 0,
                        modified: None,
                    }],
                    file_count: 1,
                    folder_count: 0,
                    modified: None,
                },
                FsNode {
                    name: "empty".into(),
                    path: PathBuf::from("/root/empty"),
                    is_dir: true,
                    current_size: 0,
                    prev_size: None,
                    children: vec![],
                    file_count: 0,
                    folder_count: 0,
                    modified: None,
                },
            ],
            file_count: 1,
            folder_count: 2,
            modified: None,
        }]
    }

    fn make_db_nodes() -> Vec<DbNode> {
        vec![
            DbNode {
                id: 1,
                scan_id: 1,
                parent_id: None,
                name: "root".into(),
                path: "/root".into(),
                is_dir: true,
                size: 300,
                file_count: 1,
                folder_count: 2,
                modified: None,
            },
            DbNode {
                id: 2,
                scan_id: 1,
                parent_id: Some(1),
                name: "docs".into(),
                path: "/root/docs".into(),
                is_dir: true,
                size: 200,
                file_count: 1,
                folder_count: 0,
                modified: None,
            },
            DbNode {
                id: 3,
                scan_id: 1,
                parent_id: Some(2),
                name: "readme.txt".into(),
                path: "/root/docs/readme.txt".into(),
                is_dir: false,
                size: 200,
                file_count: 0,
                folder_count: 0,
                modified: None,
            },
        ]
    }

    // -----------------------------------------------------------------------
    // flatten_tree tests
    // -----------------------------------------------------------------------

    #[test]
    fn flatten_collapsed_returns_only_roots() {
        let tree = make_tree();
        let expanded = HashSet::new();
        let flat = flatten_tree(&tree, &expanded);
        assert_eq!(flat.len(), 1, "collapsed tree must show only the root");
        assert_eq!(flat[0].fs_node.name, "root");
        assert_eq!(flat[0].depth, 0);
        assert!(!flat[0].expanded);
    }

    #[test]
    fn flatten_expanding_root_surfaces_direct_children() {
        let tree = make_tree();
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));

        let flat = flatten_tree(&tree, &expanded);
        // root + docs + empty = 3 (readme.txt is inside collapsed docs)
        assert_eq!(flat.len(), 3);
        assert_eq!(flat[0].depth, 0); // root
        assert_eq!(flat[1].depth, 1); // docs
        assert_eq!(flat[2].depth, 1); // empty
        assert!(flat[0].expanded);
        assert!(!flat[1].expanded);
    }

    #[test]
    fn flatten_expanding_nested_node_surfaces_grandchildren() {
        let tree = make_tree();
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));
        expanded.insert(PathBuf::from("/root/docs"));

        let flat = flatten_tree(&tree, &expanded);
        // root + docs + readme.txt + empty = 4
        assert_eq!(flat.len(), 4);
        let readme = &flat[2];
        assert_eq!(readme.fs_node.name, "readme.txt");
        assert_eq!(readme.depth, 2);
    }

    #[test]
    fn flatten_scan_progress_largest_sibling_is_one() {
        let tree = make_tree();
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));

        let flat = flatten_tree(&tree, &expanded);
        // docs (200) is the largest child → progress == 1.0
        let docs = flat.iter().find(|n| n.fs_node.name == "docs").unwrap();
        assert!((docs.scan_progress - 1.0).abs() < f32::EPSILON);

        // empty (0) → progress == 0.0
        let empty = flat.iter().find(|n| n.fs_node.name == "empty").unwrap();
        assert!((empty.scan_progress).abs() < f32::EPSILON);
    }

    // -----------------------------------------------------------------------
    // build_baseline_map tests
    // -----------------------------------------------------------------------

    #[test]
    fn build_baseline_map_correct_entries() {
        let db_nodes = make_db_nodes();
        let map = build_baseline_map(&db_nodes);

        assert_eq!(map.get(&PathBuf::from("/root")), Some(&300));
        assert_eq!(map.get(&PathBuf::from("/root/docs")), Some(&200));
        assert_eq!(map.get(&PathBuf::from("/root/docs/readme.txt")), Some(&200));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn build_baseline_map_empty_input() {
        let map = build_baseline_map(&[]);
        assert!(map.is_empty());
    }

    // -----------------------------------------------------------------------
    // merge_baseline tests
    // -----------------------------------------------------------------------

    #[test]
    fn merge_baseline_populates_matching_paths() {
        let mut tree = make_tree();
        let db_nodes = make_db_nodes();
        let baseline = build_baseline_map(&db_nodes);

        merge_baseline(&mut tree, &baseline);

        let root = &tree[0];
        assert_eq!(root.prev_size, Some(300));

        let docs = &root.children[0];
        assert_eq!(docs.prev_size, Some(200));

        let readme = &docs.children[0];
        assert_eq!(readme.prev_size, Some(200));
    }

    #[test]
    fn merge_baseline_unmatched_paths_stay_none() {
        let mut tree = make_tree();
        // Baseline has no matching paths.
        let baseline = HashMap::new();
        merge_baseline(&mut tree, &baseline);

        let root = &tree[0];
        assert_eq!(root.prev_size, None);
        assert_eq!(root.children[0].prev_size, None);
    }

    #[test]
    fn merge_baseline_partial_match() {
        let mut tree = make_tree();
        let mut baseline = HashMap::new();
        // Only the root is in the baseline.
        baseline.insert(PathBuf::from("/root"), 250_u64);

        merge_baseline(&mut tree, &baseline);

        assert_eq!(tree[0].prev_size, Some(250));
        // Children have no baseline entry.
        assert_eq!(tree[0].children[0].prev_size, None);
    }

    // -----------------------------------------------------------------------
    // scan_dir_sync tests (real filesystem via tempfile)
    // -----------------------------------------------------------------------

    #[test]
    fn scan_dir_sync_sums_children_bottom_up() {
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();

        // Write known file sizes.
        std::fs::write(root.join("a.txt"), vec![0u8; 100]).unwrap();

        let sub = root.join("sub");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(sub.join("b.txt"), vec![0u8; 200]).unwrap();

        let node = scan_dir_sync(root);

        // sub/b.txt
        let sub_node = node.children.iter().find(|c| c.name == "sub").unwrap();
        let b_node = sub_node.children.iter().find(|c| c.name == "b.txt").unwrap();
        assert_eq!(b_node.current_size, 200);

        // sub directory = sum of its children
        assert_eq!(sub_node.current_size, 200);

        // root = a.txt + sub
        assert_eq!(node.current_size, 300);
    }

    #[test]
    fn scan_dir_sync_single_file() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.path().join("only.txt"), vec![0u8; 42]).unwrap();

        let node = scan_dir_sync(dir.path());
        let file_node = node.children.iter().find(|c| c.name == "only.txt").unwrap();
        assert_eq!(file_node.current_size, 42);
        assert_eq!(node.current_size, 42);
    }

    #[test]
    fn scan_dir_sync_empty_dir() {
        let dir = tempfile::TempDir::new().unwrap();
        let node = scan_dir_sync(dir.path());
        assert_eq!(node.current_size, 0);
        assert!(node.children.is_empty());
    }

    #[test]
    fn scan_dir_sync_nested_dirs() {
        let dir = tempfile::TempDir::new().unwrap();
        let a = dir.path().join("a");
        let b = a.join("b");
        std::fs::create_dir_all(&b).unwrap();
        std::fs::write(b.join("deep.txt"), vec![0u8; 512]).unwrap();

        let node = scan_dir_sync(dir.path());
        assert_eq!(node.current_size, 512, "sizes should propagate through all levels");
    }

    #[test]
    fn scan_dir_sync_file_and_folder_counts() {
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();

        std::fs::write(root.join("a.txt"), b"a").unwrap();
        std::fs::write(root.join("b.txt"), b"b").unwrap();

        let sub = root.join("sub");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(sub.join("c.txt"), b"c").unwrap();

        let node = scan_dir_sync(root);

        // root has 3 files total (a.txt, b.txt, sub/c.txt) and 1 folder (sub)
        assert_eq!(node.file_count, 3);
        assert_eq!(node.folder_count, 1);

        let sub_node = node.children.iter().find(|c| c.name == "sub").unwrap();
        assert_eq!(sub_node.file_count, 1);
        assert_eq!(sub_node.folder_count, 0);

        // File entries have 0 counts
        let a_node = node.children.iter().find(|c| c.name == "a.txt").unwrap();
        assert_eq!(a_node.file_count, 0);
        assert_eq!(a_node.folder_count, 0);
    }

    #[test]
    fn scan_dir_sync_modified_timestamps_present() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.path().join("file.txt"), b"data").unwrap();

        let node = scan_dir_sync(dir.path());
        // The directory itself should have a modified timestamp
        assert!(node.modified.is_some(), "directory should have modified timestamp");

        let file_node = node.children.iter().find(|c| c.name == "file.txt").unwrap();
        assert!(file_node.modified.is_some(), "file should have modified timestamp");

        // Timestamps should be ISO 8601 format
        let ts = file_node.modified.as_ref().unwrap();
        assert!(ts.contains('T') && ts.ends_with('Z'), "timestamp should be ISO 8601: {ts}");
    }
}
