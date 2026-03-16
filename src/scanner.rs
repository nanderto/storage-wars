use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use crate::models::{DbNode, FsNode, UiNode};

// ---------------------------------------------------------------------------
// Incremental scan messages
// ---------------------------------------------------------------------------

/// Messages sent from scanner worker threads to the UI thread.
#[derive(Debug)]
pub enum ScanMessage {
    /// A directory was read; here are its immediate children.
    DirScanned {
        parent_path: PathBuf,
        children: Vec<FsNode>,
    },
    /// A directory could not be read.
    ScanError {
        path: PathBuf,
        error: String,
    },
    /// All workers finished — the scan is complete.
    Complete,
}

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
// Incremental parallel scanner
// ---------------------------------------------------------------------------

/// Read ONE directory's immediate contents without recursing.
/// Subdirectories get `current_size: 0` and empty `children` (they'll be scanned later).
pub fn read_dir_immediate(dir: &Path) -> std::io::Result<Vec<FsNode>> {
    let mut children = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());

        let meta = fs::metadata(&path);
        let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let modified = meta
            .as_ref()
            .ok()
            .and_then(|m| m.modified().ok())
            .map(format_system_time);

        if is_dir {
            children.push(FsNode {
                name,
                path,
                is_dir: true,
                current_size: 0,
                prev_size: None,
                children: vec![],
                file_count: 0,
                folder_count: 0,
                modified,
            });
        } else {
            let size = meta.map(|m| m.len()).unwrap_or(0);
            children.push(FsNode {
                name,
                path,
                is_dir: false,
                current_size: size,
                prev_size: None,
                children: vec![],
                file_count: 0,
                folder_count: 0,
                modified,
            });
        }
    }
    Ok(children)
}

/// Run an incremental parallel scan of `root`, sending results through `tx`.
///
/// Spawns `num_workers` threads that pull directories from a shared work queue.
/// Each worker reads one directory, sends its children via the channel, and
/// pushes subdirectories back onto the queue.
///
/// Set `cancelled` to `true` to stop workers after their current directory.
/// Sends `ScanMessage::Complete` after all workers have finished.
pub fn scan_dir_incremental(
    root: PathBuf,
    tx: async_channel::Sender<ScanMessage>,
    cancelled: Arc<AtomicBool>,
    num_workers: usize,
) {
    let queue = Arc::new(Mutex::new(VecDeque::from([root])));
    let active_workers = Arc::new(AtomicUsize::new(0));
    let condvar = Arc::new(Condvar::new());

    std::thread::scope(|s| {
        for _ in 0..num_workers {
            let queue = Arc::clone(&queue);
            let active_workers = Arc::clone(&active_workers);
            let condvar = Arc::clone(&condvar);
            let cancelled = Arc::clone(&cancelled);
            let tx = tx.clone();

            s.spawn(move || {
                loop {
                    if cancelled.load(Ordering::Relaxed) {
                        return;
                    }

                    // Try to get work from the queue
                    let dir = {
                        let mut q = queue.lock().unwrap();
                        loop {
                            if cancelled.load(Ordering::Relaxed) {
                                return;
                            }
                            if let Some(dir) = q.pop_front() {
                                active_workers.fetch_add(1, Ordering::SeqCst);
                                break Some(dir);
                            }
                            // Queue is empty — are other workers still producing?
                            if active_workers.load(Ordering::SeqCst) == 0 {
                                // Nobody active, no work left — we're done
                                break None;
                            }
                            // Wait for new work or completion
                            q = condvar.wait(q).unwrap();
                        }
                    };

                    let dir = match dir {
                        Some(d) => d,
                        None => {
                            // Wake all waiters so they can exit too
                            condvar.notify_all();
                            return;
                        }
                    };

                    // Read this directory
                    match read_dir_immediate(&dir) {
                        Ok(children) => {
                            // Push subdirectories onto the work queue
                            {
                                let mut q = queue.lock().unwrap();
                                for child in &children {
                                    if child.is_dir {
                                        q.push_back(child.path.clone());
                                    }
                                }
                            }
                            // Notify waiters that new work may be available
                            condvar.notify_all();

                            // Send results to UI
                            let _ = tx.send_blocking(ScanMessage::DirScanned {
                                parent_path: dir,
                                children,
                            });
                        }
                        Err(e) => {
                            let _ = tx.send_blocking(ScanMessage::ScanError {
                                path: dir,
                                error: e.to_string(),
                            });
                        }
                    }

                    active_workers.fetch_sub(1, Ordering::SeqCst);
                    condvar.notify_all();
                }
            });
        }
    });

    // All workers have joined — signal completion
    let _ = tx.send_blocking(ScanMessage::Complete);
}

/// Insert `children` into the tree at `parent_path`.
/// Returns `true` if the parent was found and children were inserted.
pub fn insert_children(root: &mut FsNode, parent_path: &Path, children: Vec<FsNode>) -> bool {
    if root.path == parent_path {
        root.children = children;
        return true;
    }
    // Only descend into the one child whose path is a prefix of the target
    for child in &mut root.children {
        if child.is_dir && parent_path.starts_with(&child.path) {
            return insert_children(child, parent_path, children);
        }
    }
    false
}

/// Recalculate `current_size`, `file_count`, and `folder_count` bottom-up.
pub fn recalculate_sizes(node: &mut FsNode) {
    if !node.is_dir {
        return;
    }

    for child in &mut node.children {
        recalculate_sizes(child);
    }

    node.current_size = node.children.iter().map(|c| c.current_size).sum();
    node.file_count = node
        .children
        .iter()
        .map(|c| if c.is_dir { c.file_count } else { 1 })
        .sum();
    node.folder_count = node
        .children
        .iter()
        .map(|c| if c.is_dir { 1 + c.folder_count } else { 0 })
        .sum();
}

// ---------------------------------------------------------------------------
// Tree flattening — turns the nested FsNode tree into a flat list for the UI
// ---------------------------------------------------------------------------

/// Flatten `roots` into a `Vec<UiNode>` for rendering.
/// Only expands nodes whose path is in `expanded_paths`.
/// Children are sorted by size descending (largest first).
/// `pct_of_parent` is the actual percentage of the parent's total size.
pub fn flatten_tree(roots: &[FsNode], expanded_paths: &HashSet<PathBuf>) -> Vec<UiNode> {
    let parent_total: u64 = roots.iter().map(|n| n.current_size).sum();
    let mut sorted: Vec<&FsNode> = roots.iter().collect();
    sorted.sort_by(|a, b| b.current_size.cmp(&a.current_size));
    let mut out = Vec::new();
    for node in sorted {
        flatten_node(node, 0, parent_total, expanded_paths, &mut out);
    }
    out
}

fn flatten_node(
    node: &FsNode,
    depth: usize,
    parent_total: u64,
    expanded_paths: &HashSet<PathBuf>,
    out: &mut Vec<UiNode>,
) {
    let pct_of_parent = if parent_total == 0 {
        0.0_f32
    } else {
        (node.current_size as f64 / parent_total as f64 * 100.0) as f32
    };

    let expanded = node.is_dir && expanded_paths.contains(&node.path);

    out.push(UiNode {
        fs_node: node.clone(),
        depth,
        expanded,
        pct_of_parent,
    });

    if expanded {
        let child_total: u64 = node.children.iter().map(|c| c.current_size).sum();
        let mut sorted: Vec<&FsNode> = node.children.iter().collect();
        sorted.sort_by(|a, b| b.current_size.cmp(&a.current_size));
        for child in sorted {
            flatten_node(child, depth + 1, child_total, expanded_paths, out);
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
    fn flatten_pct_of_parent_correct() {
        let tree = make_tree();
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));

        let flat = flatten_tree(&tree, &expanded);
        // docs=200 of parent total 200 → 100.0%
        let docs = flat.iter().find(|n| n.fs_node.name == "docs").unwrap();
        assert!((docs.pct_of_parent - 100.0).abs() < 0.1);

        // empty=0 of parent total 200 → 0.0%
        let empty = flat.iter().find(|n| n.fs_node.name == "empty").unwrap();
        assert!((empty.pct_of_parent).abs() < f32::EPSILON);
    }

    #[test]
    fn flatten_children_sorted_by_size_descending() {
        let tree = make_tree();
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));

        let flat = flatten_tree(&tree, &expanded);
        // Children should be sorted: docs (200) before empty (0)
        assert_eq!(flat[1].fs_node.name, "docs");
        assert_eq!(flat[2].fs_node.name, "empty");
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

    // -----------------------------------------------------------------------
    // read_dir_immediate tests
    // -----------------------------------------------------------------------

    #[test]
    fn read_dir_immediate_files_and_dirs() {
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();
        std::fs::write(root.join("a.txt"), vec![0u8; 100]).unwrap();
        std::fs::create_dir(root.join("sub")).unwrap();

        let children = read_dir_immediate(root).unwrap();
        assert_eq!(children.len(), 2);

        let file = children.iter().find(|c| c.name == "a.txt").unwrap();
        assert!(!file.is_dir);
        assert_eq!(file.current_size, 100);

        let sub = children.iter().find(|c| c.name == "sub").unwrap();
        assert!(sub.is_dir);
        assert_eq!(sub.current_size, 0, "subdirs start at size 0");
        assert!(sub.children.is_empty(), "subdirs have no children yet");
    }

    #[test]
    fn read_dir_immediate_empty_dir() {
        let dir = tempfile::TempDir::new().unwrap();
        let children = read_dir_immediate(dir.path()).unwrap();
        assert!(children.is_empty());
    }

    #[test]
    fn read_dir_immediate_nonexistent_path() {
        let result = read_dir_immediate(Path::new("/nonexistent/path/that/does/not/exist"));
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // insert_children tests
    // -----------------------------------------------------------------------

    #[test]
    fn insert_children_at_root() {
        let mut root = FsNode {
            name: "root".into(),
            path: PathBuf::from("/root"),
            is_dir: true,
            current_size: 0,
            prev_size: None,
            children: vec![],
            file_count: 0,
            folder_count: 0,
            modified: None,
        };

        let new_children = vec![FsNode {
            name: "file.txt".into(),
            path: PathBuf::from("/root/file.txt"),
            is_dir: false,
            current_size: 42,
            prev_size: None,
            children: vec![],
            file_count: 0,
            folder_count: 0,
            modified: None,
        }];

        assert!(insert_children(&mut root, Path::new("/root"), new_children));
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].name, "file.txt");
    }

    #[test]
    fn insert_children_nested() {
        let mut root = FsNode {
            name: "root".into(),
            path: PathBuf::from("/root"),
            is_dir: true,
            current_size: 0,
            prev_size: None,
            children: vec![FsNode {
                name: "sub".into(),
                path: PathBuf::from("/root/sub"),
                is_dir: true,
                current_size: 0,
                prev_size: None,
                children: vec![],
                file_count: 0,
                folder_count: 0,
                modified: None,
            }],
            file_count: 0,
            folder_count: 0,
            modified: None,
        };

        let new_children = vec![FsNode {
            name: "deep.txt".into(),
            path: PathBuf::from("/root/sub/deep.txt"),
            is_dir: false,
            current_size: 99,
            prev_size: None,
            children: vec![],
            file_count: 0,
            folder_count: 0,
            modified: None,
        }];

        assert!(insert_children(&mut root, Path::new("/root/sub"), new_children));
        assert_eq!(root.children[0].children.len(), 1);
        assert_eq!(root.children[0].children[0].name, "deep.txt");
    }

    #[test]
    fn insert_children_missing_parent() {
        let mut root = FsNode {
            name: "root".into(),
            path: PathBuf::from("/root"),
            is_dir: true,
            current_size: 0,
            prev_size: None,
            children: vec![],
            file_count: 0,
            folder_count: 0,
            modified: None,
        };

        let result = insert_children(&mut root, Path::new("/nonexistent"), vec![]);
        assert!(!result, "should return false for missing parent");
    }

    // -----------------------------------------------------------------------
    // recalculate_sizes tests
    // -----------------------------------------------------------------------

    #[test]
    fn recalculate_sizes_bottom_up() {
        let mut root = FsNode {
            name: "root".into(),
            path: PathBuf::from("/root"),
            is_dir: true,
            current_size: 0,
            prev_size: None,
            children: vec![
                FsNode {
                    name: "a.txt".into(),
                    path: PathBuf::from("/root/a.txt"),
                    is_dir: false,
                    current_size: 100,
                    prev_size: None,
                    children: vec![],
                    file_count: 0,
                    folder_count: 0,
                    modified: None,
                },
                FsNode {
                    name: "sub".into(),
                    path: PathBuf::from("/root/sub"),
                    is_dir: true,
                    current_size: 0,
                    prev_size: None,
                    children: vec![FsNode {
                        name: "b.txt".into(),
                        path: PathBuf::from("/root/sub/b.txt"),
                        is_dir: false,
                        current_size: 200,
                        prev_size: None,
                        children: vec![],
                        file_count: 0,
                        folder_count: 0,
                        modified: None,
                    }],
                    file_count: 0,
                    folder_count: 0,
                    modified: None,
                },
            ],
            file_count: 0,
            folder_count: 0,
            modified: None,
        };

        recalculate_sizes(&mut root);

        // sub: size=200, files=1, folders=0
        let sub = root.children.iter().find(|c| c.name == "sub").unwrap();
        assert_eq!(sub.current_size, 200);
        assert_eq!(sub.file_count, 1);
        assert_eq!(sub.folder_count, 0);

        // root: size=300, files=2, folders=1
        assert_eq!(root.current_size, 300);
        assert_eq!(root.file_count, 2);
        assert_eq!(root.folder_count, 1);
    }

    // -----------------------------------------------------------------------
    // scan_dir_incremental tests
    // -----------------------------------------------------------------------

    #[test]
    fn scan_dir_incremental_discovers_all_files() {
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();

        std::fs::write(root.join("a.txt"), vec![0u8; 100]).unwrap();
        let sub = root.join("sub");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(sub.join("b.txt"), vec![0u8; 200]).unwrap();

        let (tx, rx) = async_channel::bounded(256);
        let cancelled = Arc::new(AtomicBool::new(false));

        scan_dir_incremental(root.to_path_buf(), tx, cancelled, 2);

        // Collect all messages
        let mut dir_scanned_count = 0;
        let mut got_complete = false;
        let mut all_children: Vec<String> = Vec::new();

        loop {
            match rx.try_recv() {
                Ok(ScanMessage::DirScanned { children, .. }) => {
                    dir_scanned_count += 1;
                    for c in &children {
                        all_children.push(c.name.clone());
                    }
                }
                Ok(ScanMessage::ScanError { .. }) => {}
                Ok(ScanMessage::Complete) => {
                    got_complete = true;
                    break;
                }
                Err(_) => break,
            }
        }

        assert!(got_complete, "should receive Complete message");
        assert!(dir_scanned_count >= 2, "should scan at least root + sub");
        assert!(all_children.contains(&"a.txt".to_string()));
        assert!(all_children.contains(&"b.txt".to_string()));
        assert!(all_children.contains(&"sub".to_string()));
    }

    #[test]
    fn scan_dir_incremental_cancellation() {
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();

        // Create a structure with enough depth to give us time to cancel
        let mut path = root.to_path_buf();
        for i in 0..10 {
            path = path.join(format!("dir{i}"));
            std::fs::create_dir(&path).unwrap();
            std::fs::write(path.join("file.txt"), b"data").unwrap();
        }

        let (tx, rx) = async_channel::bounded(256);
        let cancelled = Arc::new(AtomicBool::new(true)); // Cancel immediately

        scan_dir_incremental(root.to_path_buf(), tx, cancelled, 2);

        // Should still get Complete even when cancelled
        let mut got_complete = false;
        let mut dir_count = 0;
        loop {
            match rx.try_recv() {
                Ok(ScanMessage::DirScanned { .. }) => dir_count += 1,
                Ok(ScanMessage::ScanError { .. }) => {}
                Ok(ScanMessage::Complete) => {
                    got_complete = true;
                    break;
                }
                Err(_) => break,
            }
        }

        assert!(got_complete, "should receive Complete even when cancelled");
        // With immediate cancellation, should scan very few (possibly zero) dirs
        assert!(dir_count <= 2, "cancelled scan should process very few dirs, got {dir_count}");
    }
}
