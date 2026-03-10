use std::collections::HashSet;
use std::path::PathBuf;

use gpui::prelude::*;
use gpui::{
    div, px, relative, rgb, App, AsyncApp, ClickEvent, Context, Entity, Focusable, FocusHandle,
    IntoElement, Render, SharedString, WeakEntity, Window,
};
use gpui_component::TitleBar;

use crate::drive_selector::{DriveSelector, DriveSelectorEvent};
use crate::models::{format_number, format_size, DbNode, DriveInfo, FsNode};
use crate::persistence;
use crate::scan_history::{ScanHistory, ScanHistoryEvent};
use crate::scanner;
use crate::tree_view::{TreeView, TreeViewEvent};

// ---------------------------------------------------------------------------
// Root application view — wires all sub-views together
// ---------------------------------------------------------------------------

pub struct AppView {
    db: rusqlite::Connection,
    drives: Vec<DriveInfo>,
    selected_drive: Option<String>,
    expanded_paths: HashSet<PathBuf>,
    current_scan_root: Option<FsNode>,
    scanning: bool,
    scan_item_count: Option<u64>,
    last_scan_time: Option<String>,
    scan_status: SharedString,

    drive_selector: Entity<DriveSelector>,
    scan_history: Entity<ScanHistory>,
    tree_view: Entity<TreeView>,
    focus_handle: FocusHandle,
}

impl AppView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let db = persistence::open_db().expect("failed to open database");

        let drive_selector = cx.new(|cx| DriveSelector::new(window, cx));
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
            drives: Vec::new(),
            selected_drive: None,
            expanded_paths: HashSet::new(),
            current_scan_root: None,
            scanning: false,
            scan_item_count: None,
            last_scan_time: None,
            scan_status: "Ready".into(),
            drive_selector,
            scan_history,
            tree_view,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Populate the drive list in the DriveSelector panel.
    pub fn set_drives(
        &mut self,
        drives: Vec<DriveInfo>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ds = self.drive_selector.clone();
        ds.update(cx, |v, cx| v.set_drives(drives.clone(), window, cx));
        self.drives = drives;
    }

    // -----------------------------------------------------------------------
    // Helpers for selected drive info
    // -----------------------------------------------------------------------

    fn selected_drive_info(&self) -> Option<&DriveInfo> {
        let drive = self.selected_drive.as_ref()?;
        self.drives.iter().find(|d| &d.name == drive)
    }

    // -----------------------------------------------------------------------
    // Event handlers
    // -----------------------------------------------------------------------

    fn on_drive_selected(&mut self, drive: String, cx: &mut Context<Self>) {
        self.selected_drive = Some(drive.clone());
        self.expanded_paths.clear();
        self.current_scan_root = None;
        self.scan_item_count = None;
        self.last_scan_time = None;

        // Load most recent scan for this drive if one exists
        if let Ok(scans) = persistence::get_scans_for_drive(&self.db, &drive) {
            let sh = self.scan_history.clone();
            sh.update(cx, |v, cx| v.set_scans(scans.clone(), cx));

            // If there's a previous scan, load it into the tree immediately
            if let Some(latest) = scans.first() {
                if let Ok(nodes) = persistence::load_scan_tree(&self.db, latest.id) {
                    if let Some(root) = build_fs_tree(&nodes) {
                        self.scan_item_count = Some(root.file_count + root.folder_count);
                        self.last_scan_time = Some(latest.scanned_at.clone());
                        self.current_scan_root = Some(root);
                        self.scan_status = "Ready".into();
                        self.rebuild_tree(cx);
                        return;
                    }
                }
            }
        }

        // No previous scan — auto-start one
        let tv = self.tree_view.clone();
        tv.update(cx, |v, cx| v.set_nodes(vec![], cx));
        self.start_scan(cx);
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

        let root = roots.remove(0);
        self.scan_item_count = Some(root.file_count + root.folder_count);
        self.current_scan_root = Some(root);
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
        self.scan_status = "Scanning…".into();
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
                view.scan_item_count = Some(root.file_count + root.folder_count);
                if persistence::save_scan(&view.db, &drive_for_save, &root).is_ok() {
                    if let Ok(scans) =
                        persistence::get_scans_for_drive(&view.db, &drive_for_save)
                    {
                        view.last_scan_time =
                            scans.first().map(|s| s.scanned_at.clone());
                        let sh = view.scan_history.clone();
                        sh.update(cx, |v: &mut ScanHistory, cx| v.set_scans(scans, cx));
                    }
                }
                view.current_scan_root = Some(root);
                view.expanded_paths.clear();
                view.scanning = false;
                view.scan_status = "Scan complete".into();
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
        let scan_label: &str = if scanning { "Scanning…" } else { "Scan" };

        let accent = rgb(0x89b4fa);
        let dim = rgb(0x6c7086);
        let normal = rgb(0xcdd6f4);
        let border = rgb(0x313244);

        // Drive info panel (WizTree-style: Selection, Total Space, Space Used, Space Free)
        let drive_info = self.selected_drive_info().cloned();
        let drive_info_panel = if let Some(ref di) = drive_info {
            let used = di.total_space.saturating_sub(di.available_space);
            let used_pct = if di.total_space > 0 {
                used as f64 / di.total_space as f64 * 100.0
            } else {
                0.0
            };
            let free_pct = 100.0 - used_pct;
            let selection_label = if di.volume_label.is_empty() {
                format!("[{}]", di.name)
            } else {
                format!("[{}]  {}", di.name, di.volume_label)
            };

            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .pl_4()
                .text_xs()
                .child(
                    div().flex().gap_2()
                        .child(div().w(px(80.)).text_color(dim).child("Selection:"))
                        .child(div().text_color(normal).font_weight(gpui::FontWeight::BOLD).child(selection_label)),
                )
                .child(
                    div().flex().gap_2()
                        .child(div().w(px(80.)).text_color(dim).child("Total Space:"))
                        .child(div().text_color(normal).font_weight(gpui::FontWeight::BOLD).child(format_size(di.total_space))),
                )
                .child(
                    div().flex().gap_2()
                        .child(div().w(px(80.)).text_color(dim).child("Space Used:"))
                        .child(div().text_color(normal).font_weight(gpui::FontWeight::BOLD).child(
                            format!("{}  ({:.1}%)", format_size(used), used_pct)
                        )),
                )
                .child(
                    div().flex().gap_2()
                        .child(div().w(px(80.)).text_color(dim).child("Space Free:"))
                        .child(div().text_color(normal).font_weight(gpui::FontWeight::BOLD).child(
                            format!("{}  ({:.1}%)", format_size(di.available_space), free_pct)
                        )),
                )
        } else {
            div()
        };

        // Status bar content
        let status_items = self
            .scan_item_count
            .map(|c| format!("{} items", format_number(c)))
            .unwrap_or_default();
        let status_drive = self
            .selected_drive
            .clone()
            .unwrap_or_default();
        let status_time = self
            .last_scan_time
            .clone()
            .map(|t| format!("Last scan: {t}"))
            .unwrap_or_default();

        let scan_status_text = self.scan_status.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            // Row 1: Title bar — use TitleBar's built-in window controls
            .child(
                TitleBar::new().bg(rgb(0x181825)).border_color(border).child(
                    div()
                        .flex()
                        .items_center()
                        .w_full()
                        .gap_3()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(accent)
                                .child("Storage Wars"),
                        )
                        // Gear icon for settings (placeholder)
                        .child(
                            div()
                                .id("btn-settings")
                                .px_2()
                                .py_0p5()
                                .rounded_sm()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0x313244)))
                                .text_color(dim)
                                .text_sm()
                                .child("\u{2699}"),
                        ),
                ),
            )
            // Row 2: Toolbar
            .child(
                div()
                    .flex()
                    .items_start()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x181825))
                    .border_b_1()
                    .border_color(border)
                    // Left group: label + dropdown + scan button
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .flex_shrink_0()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(dim)
                                            .child("Select:"),
                                    )
                                    .child(
                                        div()
                                            .w(px(200.))
                                            .child(self.drive_selector.clone()),
                                    )
                                    .child(
                                        div()
                                            .id("scan-now")
                                            .px_4()
                                            .py_1()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .when(has_drive && !scanning, |el| {
                                                el.bg(accent)
                                            })
                                            .when(!has_drive || scanning, |el| {
                                                el.bg(rgb(0x313244))
                                            })
                                            .text_color(if has_drive && !scanning {
                                                rgb(0x1e1e2e)
                                            } else {
                                                dim
                                            })
                                            .text_sm()
                                            .child(scan_label)
                                            .on_click(cx.listener(
                                                |this, _: &ClickEvent, _window, cx| {
                                                    this.start_scan(cx);
                                                },
                                            )),
                                    ),
                            )
                            // Progress bar — always visible below the controls
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(dim)
                                            .child(scan_status_text),
                                    )
                                    .child(
                                        div()
                                            .w(px(200.))
                                            .h(px(6.))
                                            .rounded_sm()
                                            .bg(rgb(0x313244))
                                            .overflow_hidden()
                                            .when(scanning, |el| {
                                                // Indeterminate: fill 40% with animation feel
                                                el.child(
                                                    div()
                                                        .h_full()
                                                        .w(relative(0.4))
                                                        .rounded_sm()
                                                        .bg(accent),
                                                )
                                            })
                                            .when(!scanning && has_drive && self.current_scan_root.is_some(), |el| {
                                                // Complete: full green bar
                                                el.child(
                                                    div()
                                                        .h_full()
                                                        .w_full()
                                                        .rounded_sm()
                                                        .bg(rgb(0x22c55e)),
                                                )
                                            }),
                                    ),
                            ),
                    )
                    // Right group: drive info panel
                    .child(drive_info_panel),
            )
            // Row 3: Main content — tree view fills the area
            .child(
                div()
                    .flex()
                    .flex_grow()
                    .min_h_0()
                    .child(self.tree_view.clone()),
            )
            // Row 4: Status bar
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .h(px(24.))
                    .bg(rgb(0x181825))
                    .border_t_1()
                    .border_color(border)
                    .text_xs()
                    .text_color(dim)
                    .child(div().child(status_items))
                    .child(div().child(status_drive))
                    .child(div().child(status_time)),
            )
    }
}
