use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use gpui::{AsyncApp, Context, EventEmitter, WeakEntity};

use crate::models::{FsNode, UiNode};
use crate::scanner;

// ---------------------------------------------------------------------------
// Events emitted by the scanner entity
// ---------------------------------------------------------------------------

pub enum ScannerEvent {
    /// Emitted every ~200ms during scan with current progress.
    Progress,
    /// Emitted once when scan finishes.
    Complete { was_cancelled: bool },
}

impl EventEmitter<ScannerEvent> for DirectoryScanner {}

// ---------------------------------------------------------------------------
// DirectoryScanner entity — owns scan state, runs async work
// ---------------------------------------------------------------------------

pub struct DirectoryScanner {
    /// O(1) collection during scan — maps parent_path to its children.
    children_map: HashMap<PathBuf, Vec<FsNode>>,
    root_path: Option<PathBuf>,
    root_display_name: Option<String>,
    pub dirs_scanned: usize,
    pub is_scanning: bool,
    cancel: Arc<AtomicBool>,
    finished_tree: Option<FsNode>,
}

impl DirectoryScanner {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            children_map: HashMap::new(),
            root_path: None,
            root_display_name: None,
            dirs_scanned: 0,
            is_scanning: false,
            cancel: Arc::new(AtomicBool::new(false)),
            finished_tree: None,
        }
    }

    pub fn start_scan(
        &mut self,
        root_path: PathBuf,
        display_name: String,
        cx: &mut Context<Self>,
    ) {
        self.children_map.clear();
        self.root_path = Some(root_path.clone());
        self.root_display_name = Some(display_name);
        self.dirs_scanned = 0;
        self.is_scanning = true;
        self.finished_tree = None;

        let cancel = Arc::new(AtomicBool::new(false));
        self.cancel = Arc::clone(&cancel);

        let (tx, rx) = async_channel::bounded(256);
        let num_workers = std::thread::available_parallelism()
            .map(|n| n.get().min(8))
            .unwrap_or(4);

        std::thread::spawn(move || {
            scanner::scan_dir_incremental(root_path, tx, cancel, num_workers);
        });

        let bg = cx.background_executor().clone();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let mut last_emit = Instant::now();
            loop {
                let first = match rx.recv().await {
                    Ok(msg) => msg,
                    Err(_) => break,
                };

                let mut batch = vec![first];
                while batch.len() < 500 {
                    match rx.try_recv() {
                        Ok(msg) => batch.push(msg),
                        Err(_) => break,
                    }
                }

                let mut got_complete = false;
                let result = this.update(cx, |scanner, cx| {
                    for msg in batch {
                        match msg {
                            scanner::ScanMessage::DirScanned {
                                parent_path,
                                children,
                            } => {
                                scanner.children_map.insert(parent_path, children);
                                scanner.dirs_scanned += 1;
                            }
                            scanner::ScanMessage::ScanError { .. } => {}
                            scanner::ScanMessage::Complete => {
                                got_complete = true;
                            }
                        }
                    }

                    let now = Instant::now();
                    if now.duration_since(last_emit) >= Duration::from_millis(200) {
                        last_emit = now;
                        cx.emit(ScannerEvent::Progress);
                    }
                });

                if result.is_err() {
                    break;
                }

                if got_complete {
                    this.update(cx, |scanner, cx| {
                        let was_cancelled = scanner.cancel.load(Ordering::SeqCst);

                        if let (Some(name), Some(path)) = (
                            scanner.root_display_name.clone(),
                            scanner.root_path.clone(),
                        ) {
                            let mut root = FsNode {
                                name,
                                path,
                                is_dir: true,
                                current_size: 0,
                                prev_size: None,
                                children: vec![],
                                file_count: 0,
                                folder_count: 0,
                                modified: None,
                            };
                            scanner::assemble_tree(&mut root, &mut scanner.children_map);
                            scanner::recalculate_sizes(&mut root);
                            scanner.finished_tree = Some(root);
                        }

                        scanner.is_scanning = false;
                        cx.emit(ScannerEvent::Complete { was_cancelled });
                    })
                    .ok();
                    break;
                }

                bg.timer(Duration::from_millis(1)).await;
            }
        })
        .detach();
    }

    pub fn cancel(&mut self) {
        self.cancel.store(true, Ordering::SeqCst);
    }

    /// Take the finished tree out of the scanner (for AppView to own after completion).
    pub fn take_tree(&mut self) -> Option<FsNode> {
        self.finished_tree.take()
    }

    /// Walk the HashMap to produce visible nodes during scan.
    /// Only follows expanded paths. Cost: O(visible_nodes).
    pub fn flatten_visible(&self, expanded_paths: &HashSet<PathBuf>) -> Vec<UiNode> {
        let root_path = match &self.root_path {
            Some(p) => p,
            None => return vec![],
        };
        let root_name = match &self.root_display_name {
            Some(n) => n,
            None => return vec![],
        };

        let mut out = Vec::new();
        self.flatten_map_node(root_path, root_name, 0, expanded_paths, &mut out);
        out
    }

    fn flatten_map_node(
        &self,
        path: &Path,
        name: &str,
        depth: usize,
        expanded_paths: &HashSet<PathBuf>,
        out: &mut Vec<UiNode>,
    ) {
        let children = self.children_map.get(path);
        let is_dir = children.is_some() || (depth == 0);
        let expanded = is_dir && expanded_paths.contains(path);

        let node = FsNode {
            name: name.to_string(),
            path: path.to_path_buf(),
            is_dir,
            current_size: 0,
            prev_size: None,
            children: vec![],
            file_count: 0,
            folder_count: 0,
            modified: None,
        };
        out.push(UiNode {
            fs_node: node,
            depth,
            expanded,
            pct_of_parent: 0.0,
        });

        if expanded {
            if let Some(kids) = children {
                for child in kids {
                    if child.is_dir {
                        self.flatten_map_node(
                            &child.path,
                            &child.name,
                            depth + 1,
                            expanded_paths,
                            out,
                        );
                    } else {
                        out.push(UiNode {
                            fs_node: child.clone(),
                            depth: depth + 1,
                            expanded: false,
                            pct_of_parent: 0.0,
                        });
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Helper: build a DirectoryScanner with a pre-populated children_map
    /// (no gpui context needed for flatten_visible tests).
    fn make_scanner_with_map() -> DirectoryScanner {
        let root = PathBuf::from("/root");
        let sub_a = root.join("sub_a");
        let sub_b = root.join("sub_b");
        let nested = sub_a.join("nested");

        let mut children_map: HashMap<PathBuf, Vec<FsNode>> = HashMap::new();

        // Root has two subdirs and one file
        children_map.insert(
            root.clone(),
            vec![
                FsNode {
                    name: "sub_a".into(),
                    path: sub_a.clone(),
                    is_dir: true,
                    current_size: 0,
                    prev_size: None,
                    children: vec![],
                    file_count: 0,
                    folder_count: 0,
                    modified: None,
                },
                FsNode {
                    name: "sub_b".into(),
                    path: sub_b.clone(),
                    is_dir: true,
                    current_size: 0,
                    prev_size: None,
                    children: vec![],
                    file_count: 0,
                    folder_count: 0,
                    modified: None,
                },
                FsNode {
                    name: "file.txt".into(),
                    path: root.join("file.txt"),
                    is_dir: false,
                    current_size: 100,
                    prev_size: None,
                    children: vec![],
                    file_count: 0,
                    folder_count: 0,
                    modified: None,
                },
            ],
        );

        // sub_a has a nested dir and a file
        children_map.insert(
            sub_a,
            vec![
                FsNode {
                    name: "nested".into(),
                    path: nested,
                    is_dir: true,
                    current_size: 0,
                    prev_size: None,
                    children: vec![],
                    file_count: 0,
                    folder_count: 0,
                    modified: None,
                },
                FsNode {
                    name: "a.txt".into(),
                    path: PathBuf::from("/root/sub_a/a.txt"),
                    is_dir: false,
                    current_size: 50,
                    prev_size: None,
                    children: vec![],
                    file_count: 0,
                    folder_count: 0,
                    modified: None,
                },
            ],
        );

        DirectoryScanner {
            children_map,
            root_path: Some(root),
            root_display_name: Some("root".into()),
            dirs_scanned: 3,
            is_scanning: false,
            cancel: Arc::new(AtomicBool::new(false)),
            finished_tree: None,
        }
    }

    #[test]
    fn flatten_visible_empty_scanner() {
        let scanner = DirectoryScanner {
            children_map: HashMap::new(),
            root_path: None,
            root_display_name: None,
            dirs_scanned: 0,
            is_scanning: false,
            cancel: Arc::new(AtomicBool::new(false)),
            finished_tree: None,
        };
        let nodes = scanner.flatten_visible(&HashSet::new());
        assert!(nodes.is_empty(), "empty scanner should return no nodes");
    }

    #[test]
    fn flatten_visible_collapsed_root() {
        let scanner = make_scanner_with_map();
        let nodes = scanner.flatten_visible(&HashSet::new());
        assert_eq!(nodes.len(), 1, "collapsed root = 1 node");
        assert_eq!(nodes[0].fs_node.name, "root");
        assert_eq!(nodes[0].depth, 0);
        assert!(!nodes[0].expanded);
    }

    #[test]
    fn flatten_visible_expanded_root() {
        let scanner = make_scanner_with_map();
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));

        let nodes = scanner.flatten_visible(&expanded);
        // root + sub_a + sub_b + file.txt = 4
        assert_eq!(nodes.len(), 4, "expanded root = root + 3 children");
        assert!(nodes[0].expanded);
        assert_eq!(nodes[0].depth, 0);

        let names: Vec<&str> = nodes[1..].iter().map(|n| n.fs_node.name.as_str()).collect();
        assert!(names.contains(&"sub_a"), "got: {names:?}");
        assert!(names.contains(&"sub_b"), "got: {names:?}");
        assert!(names.contains(&"file.txt"), "got: {names:?}");

        for n in &nodes[1..] {
            assert_eq!(n.depth, 1);
        }
    }

    #[test]
    fn flatten_visible_nested_expansion() {
        let scanner = make_scanner_with_map();
        let mut expanded = HashSet::new();
        expanded.insert(PathBuf::from("/root"));
        expanded.insert(PathBuf::from("/root/sub_a"));

        let nodes = scanner.flatten_visible(&expanded);
        // root + sub_a(expanded) + nested + a.txt + sub_b + file.txt = 6
        assert_eq!(
            nodes.len(),
            6,
            "nested expansion: got {:?}",
            nodes.iter().map(|n| (&n.fs_node.name, n.depth)).collect::<Vec<_>>()
        );

        // Verify depths
        let depth2: Vec<&str> = nodes
            .iter()
            .filter(|n| n.depth == 2)
            .map(|n| n.fs_node.name.as_str())
            .collect();
        assert!(depth2.contains(&"nested"), "got: {depth2:?}");
        assert!(depth2.contains(&"a.txt"), "got: {depth2:?}");
    }

    // -----------------------------------------------------------------------
    // Integration tests — need gpui context for cx.spawn in start_scan.
    // DirectoryScanner doesn't implement Render, so we test it through
    // AppView (the real integration point) in app_view::tests.
    // The unit tests above cover flatten_visible thoroughly.
    // -----------------------------------------------------------------------

    #[test]
    fn take_tree_returns_none_before_scan() {
        let mut scanner = DirectoryScanner {
            children_map: HashMap::new(),
            root_path: None,
            root_display_name: None,
            dirs_scanned: 0,
            is_scanning: false,
            cancel: Arc::new(AtomicBool::new(false)),
            finished_tree: None,
        };
        assert!(scanner.take_tree().is_none(), "no tree before scan");
    }

    #[test]
    fn take_tree_returns_none_after_taken() {
        let mut scanner = DirectoryScanner {
            children_map: HashMap::new(),
            root_path: None,
            root_display_name: None,
            dirs_scanned: 0,
            is_scanning: false,
            cancel: Arc::new(AtomicBool::new(false)),
            finished_tree: Some(FsNode {
                name: "root".into(),
                path: PathBuf::from("/root"),
                is_dir: true,
                current_size: 100,
                prev_size: None,
                children: vec![],
                file_count: 1,
                folder_count: 0,
                modified: None,
            }),
        };

        let first = scanner.take_tree();
        assert!(first.is_some(), "first take should return the tree");

        let second = scanner.take_tree();
        assert!(second.is_none(), "second take should return None");
    }
}
