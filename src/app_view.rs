use std::collections::HashSet;
use std::path::PathBuf;

use gpui::prelude::*;
use gpui::{
    div, px, rgb, App, AsyncApp, ClickEvent, Context, Entity, Focusable, FocusHandle, IntoElement,
    Render, WeakEntity, Window,
};

use crate::drive_selector::{DriveSelector, DriveSelectorEvent};
use crate::models::{DbNode, DriveInfo, FsNode};
use crate::persistence;
use crate::scan_history::{ScanHistory, ScanHistoryEvent};
use crate::scanner;
use crate::tree_view::{TreeView, TreeViewEvent};

// ---------------------------------------------------------------------------
// Root application view — wires all sub-views together
// ---------------------------------------------------------------------------

pub struct AppView {
    db: rusqlite::Connection,
    selected_drive: Option<String>,
    expanded_paths: HashSet<PathBuf>,
    current_scan_root: Option<FsNode>,
    scanning: bool,

    drive_selector: Entity<DriveSelector>,
    scan_history: Entity<ScanHistory>,
    tree_view: Entity<TreeView>,
    focus_handle: FocusHandle,
}

impl AppView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let db = persistence::open_db().expect("failed to open database");

        let drive_selector = cx.new(DriveSelector::new);
        let scan_history = cx.new(ScanHistory::new);
        let tree_view = cx.new(TreeView::new);

        cx.subscribe(&drive_selector, |this, _, event, cx| {
            let DriveSelectorEvent::DriveSelected(drive) = event;
            this.on_drive_selected(drive.clone(), cx);
        })
        .detach();

        cx.subscribe(&scan_history, |this, _, event, cx| match event {
            ScanHistoryEvent::CompareRequested { base_id, new_id } => {
                this.on_compare_requested(*base_id, *new_id, cx);
            }
            ScanHistoryEvent::DeleteRequested(scan_id) => {
                this.on_delete_scan(*scan_id, cx);
            }
        })
        .detach();

        cx.subscribe(&tree_view, |this, _, event, cx| {
            let TreeViewEvent::ToggleExpand(path) = event;
            this.on_toggle_expand(path.clone(), cx);
        })
        .detach();

        Self {
            db,
            selected_drive: None,
            expanded_paths: HashSet::new(),
            current_scan_root: None,
            scanning: false,
            drive_selector,
            scan_history,
            tree_view,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Populate the drive list in the DriveSelector panel.
    pub fn set_drives(&mut self, drives: Vec<DriveInfo>, cx: &mut Context<Self>) {
        let ds = self.drive_selector.clone();
        ds.update(cx, |v, cx| v.set_drives(drives, cx));
    }

    // -----------------------------------------------------------------------
    // Event handlers
    // -----------------------------------------------------------------------

    fn on_drive_selected(&mut self, drive: String, cx: &mut Context<Self>) {
        self.selected_drive = Some(drive.clone());
        self.expanded_paths.clear();
        self.current_scan_root = None;

        if let Ok(scans) = persistence::get_scans_for_drive(&self.db, &drive) {
            let sh = self.scan_history.clone();
            sh.update(cx, |v, cx| v.set_scans(scans, cx));
        }

        let tv = self.tree_view.clone();
        tv.update(cx, |v, cx| v.set_nodes(vec![], cx));

        cx.notify();
    }

    fn on_compare_requested(&mut self, base_id: i64, new_id: i64, cx: &mut Context<Self>) {
        let base_nodes = match persistence::load_scan_tree(&self.db, base_id) {
            Ok(n) => n,
            Err(_) => return,
        };
        let new_nodes = match persistence::load_scan_tree(&self.db, new_id) {
            Ok(n) => n,
            Err(_) => return,
        };

        let root = match build_fs_tree(&new_nodes) {
            Some(r) => r,
            None => return,
        };

        let baseline = scanner::build_baseline_map(&base_nodes);
        let mut roots = vec![root];
        scanner::merge_baseline(&mut roots, &baseline);

        self.current_scan_root = Some(roots.remove(0));
        self.expanded_paths.clear();
        self.rebuild_tree(cx);
    }

    fn on_delete_scan(&mut self, scan_id: i64, cx: &mut Context<Self>) {
        let _ = persistence::delete_scan(&self.db, scan_id);
        if let Some(drive) = self.selected_drive.clone() {
            if let Ok(scans) = persistence::get_scans_for_drive(&self.db, &drive) {
                let sh = self.scan_history.clone();
                sh.update(cx, |v, cx| v.set_scans(scans, cx));
            }
        }
        cx.notify();
    }

    fn on_toggle_expand(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if self.expanded_paths.contains(&path) {
            self.expanded_paths.remove(&path);
        } else {
            self.expanded_paths.insert(path);
        }
        self.rebuild_tree(cx);
    }

    // -----------------------------------------------------------------------
    // Scan
    // -----------------------------------------------------------------------

    fn start_scan(&mut self, cx: &mut Context<Self>) {
        let drive = match self.selected_drive.clone() {
            Some(d) => d,
            None => return,
        };
        if self.scanning {
            return;
        }

        self.scanning = true;
        cx.notify();

        let path = PathBuf::from(format!("{}\\", drive));
        let drive_for_save = drive.clone();

        // Scan on a background thread, then save + refresh on the main thread.
        let task = cx
            .background_executor()
            .spawn(async move { scanner::scan_dir_sync(&path) });

        cx.spawn(async move |this: WeakEntity<AppView>, cx: &mut AsyncApp| {
            let root = task.await;
            this.update(cx, |view: &mut AppView, cx| {
                if persistence::save_scan(&view.db, &drive_for_save, &root).is_ok() {
                    if let Ok(scans) =
                        persistence::get_scans_for_drive(&view.db, &drive_for_save)
                    {
                        let sh = view.scan_history.clone();
                        sh.update(cx, |v: &mut ScanHistory, cx| v.set_scans(scans, cx));
                    }
                }
                view.current_scan_root = Some(root);
                view.expanded_paths.clear();
                view.scanning = false;
                view.rebuild_tree(cx);
            })
            .ok();
        })
        .detach();
    }

    // -----------------------------------------------------------------------
    // Tree helpers
    // -----------------------------------------------------------------------

    fn rebuild_tree(&mut self, cx: &mut Context<Self>) {
        let nodes = if let Some(root) = &self.current_scan_root {
            scanner::flatten_tree(std::slice::from_ref(root), &self.expanded_paths)
        } else {
            vec![]
        };
        let tv = self.tree_view.clone();
        tv.update(cx, |v, cx| v.set_nodes(nodes, cx));
        cx.notify();
    }
}

// ---------------------------------------------------------------------------
// Reconstruct a FsNode tree from a flat Vec<DbNode> loaded from SQLite.
// ---------------------------------------------------------------------------

fn build_fs_tree(db_nodes: &[DbNode]) -> Option<FsNode> {
    use std::collections::HashMap;

    if db_nodes.is_empty() {
        return None;
    }

    let root_db = db_nodes.iter().find(|n| n.parent_id.is_none())?;
    let id_map: HashMap<i64, &DbNode> = db_nodes.iter().map(|n| (n.id, n)).collect();
    let mut children_map: HashMap<i64, Vec<i64>> = HashMap::new();
    for n in db_nodes {
        if let Some(pid) = n.parent_id {
            children_map.entry(pid).or_default().push(n.id);
        }
    }

    Some(build_node(root_db.id, &id_map, &children_map))
}

fn build_node(
    id: i64,
    id_map: &std::collections::HashMap<i64, &DbNode>,
    children_map: &std::collections::HashMap<i64, Vec<i64>>,
) -> FsNode {
    let db_node = id_map[&id];
    let children = children_map
        .get(&id)
        .map(|ids| ids.iter().map(|&cid| build_node(cid, id_map, children_map)).collect())
        .unwrap_or_default();

    FsNode {
        name: db_node.name.clone(),
        path: PathBuf::from(&db_node.path),
        is_dir: db_node.is_dir,
        current_size: db_node.size,
        prev_size: None,
        children,
        file_count: db_node.file_count,
        folder_count: db_node.folder_count,
        modified: db_node.modified.clone(),
    }
}

// ---------------------------------------------------------------------------
// Focusable
// ---------------------------------------------------------------------------

impl Focusable for AppView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let scanning = self.scanning;
        let has_drive = self.selected_drive.is_some();
        let scan_label: &str = if scanning { "Scanning…" } else { "Scan Now" };

        div()
            .flex()
            .w_full()
            .h_full()
            .bg(rgb(0x1e1e2e))
            // Left panel: drive list
            .child(self.drive_selector.clone())
            // Middle panel: scan history
            .child(self.scan_history.clone())
            // Right area: toolbar + tree
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .h_full()
                    // Toolbar row
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .px_4()
                            .py_2()
                            .h(px(44.))
                            .bg(rgb(0x181825))
                            .border_b_1()
                            .border_color(rgb(0x313244))
                            .child(
                                div()
                                    .id("scan-now")
                                    .px_4()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .when(has_drive && !scanning, |el| el.bg(rgb(0x89b4fa)))
                                    .when(!has_drive || scanning, |el| el.bg(rgb(0x313244)))
                                    .text_color(if has_drive && !scanning {
                                        rgb(0x1e1e2e)
                                    } else {
                                        rgb(0x6c7086)
                                    })
                                    .text_sm()
                                    .child(scan_label)
                                    .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                                        this.start_scan(cx);
                                    })),
                            ),
                    )
                    // Tree view fills remaining space
                    .child(self.tree_view.clone()),
            )
    }
}
