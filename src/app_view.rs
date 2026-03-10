use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use gpui::prelude::*;
use gpui::{
    div, px, relative, rgb, App, AsyncApp, ClickEvent, Context, Entity, Focusable, FocusHandle,
    IntoElement, Render, SharedString, WeakEntity, Window,
};
use gpui::WindowControlArea;

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
    scan_completed: bool,
    scan_item_count: Option<u64>,
    last_scan_time: Option<String>,
    scan_status: SharedString,
    scan_cancel: Arc<AtomicBool>,
    dirs_scanned: usize,
    /// Override the scan root path (used for testing).
    scan_root_override: Option<PathBuf>,

    drive_selector: Entity<DriveSelector>,
    scan_history: Entity<ScanHistory>,
    tree_view: Entity<TreeView>,
    focus_handle: FocusHandle,
}

impl AppView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let db = persistence::open_db().expect("failed to open database");
        Self::new_with_db(db, window, cx)
    }

    pub fn new_with_db(
        db: rusqlite::Connection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
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
            scan_completed: false,
            scan_item_count: None,
            last_scan_time: None,
            scan_status: "Ready".into(),
            scan_cancel: Arc::new(AtomicBool::new(false)),
            dirs_scanned: 0,
            scan_root_override: None,
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
        self.scan_completed = false;

        // Load scan history sidebar and most recent scan tree if available
        if let Ok(scans) = persistence::get_scans_for_drive(&self.db, &drive) {
            let sh = self.scan_history.clone();
            sh.update(cx, |v, cx| v.set_scans(scans.clone(), cx));

            if let Some(latest) = scans.first() {
                if let Ok(nodes) = persistence::load_scan_tree(&self.db, latest.id) {
                    if let Some(root) = build_fs_tree(&nodes) {
                        self.scan_item_count = Some(root.file_count + root.folder_count);
                        self.last_scan_time = Some(latest.scanned_at.clone());
                        self.current_scan_root = Some(root);
                        self.scan_completed = true;
                        self.scan_status = "Ready".into();
                        self.rebuild_tree(cx);
                        return;
                    }
                }
            }
        }

        // No previous scan — show a single collapsed drive node
        let root_path = self
            .scan_root_override
            .clone()
            .unwrap_or_else(|| PathBuf::from(format!("{}\\", drive)));
        let display_name = self
            .scan_root_override
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| format!("{}\\", drive));

        self.current_scan_root = Some(FsNode {
            name: display_name,
            path: root_path,
            is_dir: true,
            current_size: 0,
            prev_size: None,
            children: vec![],
            file_count: 0,
            folder_count: 0,
            modified: None,
        });
        self.scan_status = "Ready".into();
        self.rebuild_tree(cx);
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

        // If already scanning, treat click as cancel
        if self.scanning {
            self.scan_cancel.store(true, Ordering::SeqCst);
            return;
        }

        // Reset cancel flag
        let cancel = Arc::new(AtomicBool::new(false));
        self.scan_cancel = Arc::clone(&cancel);
        self.scanning = true;
        self.scan_completed = false;
        self.dirs_scanned = 0;
        self.scan_status = "Scanning… (0 dirs)".into();
        self.expanded_paths.clear();

        // Show root placeholder immediately
        let root_path = self
            .scan_root_override
            .clone()
            .unwrap_or_else(|| PathBuf::from(format!("{}\\", drive)));
        let display_name = self
            .scan_root_override
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|| format!("{}\\", drive));
        let root_node = FsNode {
            name: display_name,
            path: root_path.clone(),
            is_dir: true,
            current_size: 0,
            prev_size: None,
            children: vec![],
            file_count: 0,
            folder_count: 0,
            modified: None,
        };
        self.current_scan_root = Some(root_node);
        self.rebuild_tree(cx);

        // Create channel and spawn scanner thread
        let (tx, rx) = async_channel::bounded(256);
        let num_workers = std::thread::available_parallelism()
            .map(|n| n.get().min(8))
            .unwrap_or(4);

        let scan_root = root_path;
        std::thread::spawn(move || {
            scanner::scan_dir_incremental(scan_root, tx, cancel, num_workers);
        });

        // Channel-driven UI loop: read messages directly from the channel.
        // rx.recv().await yields to gpui while the channel is empty.
        // When messages are available we drain a batch with try_recv(),
        // process it, then yield back to gpui for one frame so it can
        // render and handle input before we take the next batch.
        let drive_for_save = drive.clone();
        let bg = cx.background_executor().clone();
        cx.spawn(async move |this: WeakEntity<AppView>, cx: &mut AsyncApp| {
            loop {
                // Block (async) until the next message arrives.
                let first = match rx.recv().await {
                    Ok(msg) => msg,
                    Err(_) => break, // channel closed
                };

                // Drain whatever else is immediately available (limit 500
                // per batch so each update step stays short).
                let mut batch = vec![first];
                while batch.len() < 500 {
                    match rx.try_recv() {
                        Ok(msg) => batch.push(msg),
                        Err(_) => break,
                    }
                }

                let mut got_complete = false;
                let result = this.update(cx, |view: &mut AppView, cx| {
                    let root = match view.current_scan_root.as_mut() {
                        Some(r) => r,
                        None => return,
                    };

                    let mut visible_changed = false;
                    for msg in &batch {
                        match msg {
                            scanner::ScanMessage::DirScanned {
                                parent_path,
                                children,
                            } => {
                                scanner::insert_children(
                                    root,
                                    parent_path,
                                    children.clone(),
                                );
                                view.dirs_scanned += 1;
                                if view.expanded_paths.contains(
                                    parent_path.as_path(),
                                ) {
                                    visible_changed = true;
                                }
                            }
                            scanner::ScanMessage::ScanError { .. } => {}
                            scanner::ScanMessage::Complete => {
                                got_complete = true;
                            }
                        }
                    }

                    if visible_changed {
                        scanner::recalculate_sizes(root);
                        view.scan_item_count =
                            Some(root.file_count + root.folder_count);
                        view.rebuild_tree(cx);
                    }

                    view.scan_status = SharedString::from(format!(
                        "Scanning… ({} dirs)",
                        format_number(view.dirs_scanned as u64),
                    ));
                    cx.notify();
                });

                if result.is_err() {
                    break; // View dropped
                }

                // Yield to gpui so it can render a frame and handle input
                // before we process the next batch.  Without this, recv()
                // resolves instantly when the scanner is fast, starving the
                // render loop.
                bg.timer(std::time::Duration::from_millis(10)).await;

                if got_complete {
                    // Finalize: recalculate sizes, save to DB, rebuild tree
                    this.update(cx, |view: &mut AppView, cx| {
                        let was_cancelled =
                            view.scan_cancel.load(Ordering::SeqCst);

                        if let Some(root) = view.current_scan_root.as_mut() {
                            scanner::recalculate_sizes(root);
                            view.scan_item_count =
                                Some(root.file_count + root.folder_count);
                        }

                        if !was_cancelled {
                            if let Some(root) = &view.current_scan_root {
                                if persistence::save_scan(
                                    &view.db,
                                    &drive_for_save,
                                    root,
                                )
                                .is_ok()
                                {
                                    if let Ok(scans) =
                                        persistence::get_scans_for_drive(
                                            &view.db,
                                            &drive_for_save,
                                        )
                                    {
                                        view.last_scan_time = scans
                                            .first()
                                            .map(|s| s.scanned_at.clone());
                                        let sh = view.scan_history.clone();
                                        sh.update(cx, |v: &mut ScanHistory, cx| {
                                            v.set_scans(scans, cx)
                                        });
                                    }
                                }
                            }
                            view.scan_status = "Scan complete".into();
                        } else {
                            view.scan_status = "Scan cancelled".into();
                        }

                        view.scanning = false;
                        view.scan_completed = !was_cancelled;
                        view.rebuild_tree(cx);
                    })
                    .ok();
                    break;
                }
            }
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
        let scan_label: &str = if scanning { "Cancel" } else { "Scan" };

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
            // Row 1: Custom title bar
            .child(
                div()
                    .id("title-bar")
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(34.))
                    .bg(rgb(0x181825))
                    .border_b_1()
                    .border_color(border)
                    // Left: app title (drag area) — left padding only
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .flex_grow()
                            .h_full()
                            .pl_3()
                            .window_control_area(WindowControlArea::Drag)
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(accent)
                                    .child("Storage Wars"),
                            ),
                    )
                    // Right: gear + window controls — no right padding, flush to corner
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .h_full()
                            .flex_shrink_0()
                            // Gear (settings placeholder)
                            .child(
                                div()
                                    .id("btn-settings")
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .w(px(46.))
                                    .h_full()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x313244)))
                                    .text_color(normal)
                                    .text_sm()
                                    .child("\u{2699}"),
                            )
                            // Minimize
                            .child(
                                div()
                                    .id("btn-min")
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .w(px(46.))
                                    .h_full()
                                    .cursor_pointer()
                                    .text_color(normal)
                                    .hover(|s| s.bg(rgb(0x313244)))
                                    .text_sm()
                                    .window_control_area(WindowControlArea::Min)
                                    .child("\u{2014}"),
                            )
                            // Maximize / Restore
                            .child(
                                div()
                                    .id("btn-max")
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .w(px(46.))
                                    .h_full()
                                    .cursor_pointer()
                                    .text_color(normal)
                                    .hover(|s| s.bg(rgb(0x313244)))
                                    .text_base()
                                    .window_control_area(WindowControlArea::Max)
                                    .child("\u{25FB}"),
                            )
                            // Close — flush to top-right corner
                            .child(
                                div()
                                    .id("btn-close")
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .w(px(46.))
                                    .h_full()
                                    .cursor_pointer()
                                    .text_color(normal)
                                    .hover(|s| s.bg(rgb(0xe81123)).text_color(rgb(0xffffff)))
                                    .text_sm()
                                    .window_control_area(WindowControlArea::Close)
                                    .child("\u{2715}"),
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
                                            .when(scanning, |el| {
                                                el.bg(rgb(0xf38ba8))
                                            })
                                            .when(!has_drive && !scanning, |el| {
                                                el.bg(rgb(0x313244))
                                            })
                                            .text_color(if has_drive || scanning {
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
                                            .when(!scanning && self.scan_completed, |el| {
                                                // Complete: full bar in accent color
                                                el.child(
                                                    div()
                                                        .h_full()
                                                        .w_full()
                                                        .rounded_sm()
                                                        .bg(accent),
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a tempdir with known structure:
    ///   root/
    ///     Users/
    ///       docs/
    ///         readme.txt (100 bytes)
    ///       file.txt (50 bytes)
    ///     Windows/
    ///       system.dll (200 bytes)
    fn make_test_dir() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().unwrap();
        let root = dir.path();

        let users = root.join("Users");
        std::fs::create_dir(&users).unwrap();
        let docs = users.join("docs");
        std::fs::create_dir(&docs).unwrap();
        std::fs::write(docs.join("readme.txt"), vec![0u8; 100]).unwrap();
        std::fs::write(users.join("file.txt"), vec![0u8; 50]).unwrap();

        let windows = root.join("Windows");
        std::fs::create_dir(&windows).unwrap();
        std::fs::write(windows.join("system.dll"), vec![0u8; 200]).unwrap();

        dir
    }

    #[gpui::test]
    fn scan_button_populates_tree_with_children(cx: &mut gpui::TestAppContext) {
        cx.update(|app| gpui_component::init(app));

        let test_dir = make_test_dir();
        let test_path = test_dir.path().to_path_buf();

        let db = persistence::open_in_memory().unwrap();

        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut app = AppView::new_with_db(db, window, cx);
            app.scan_root_override = Some(test_path);
            app
        });

        // Simulate selecting a drive
        view.update(cx, |v, cx| {
            v.on_drive_selected("C:".to_string(), cx);
        });
        cx.run_until_parked();

        // Trigger scan
        view.update(cx, |v, cx| {
            v.start_scan(cx);
        });

        // Let the scanner threads finish, then process all channel messages.
        // advance_clock unblocks the yield timer between batches.
        std::thread::sleep(std::time::Duration::from_millis(200));
        cx.executor().advance_clock(std::time::Duration::from_millis(20));
        cx.run_until_parked();

        // Verify scan completed and tree has children
        view.read_with(cx, |v, _| {
            assert!(
                !v.scanning,
                "scan should have completed, status: {}",
                v.scan_status
            );
            assert!(v.scan_completed, "scan_completed should be true");

            let root = v.current_scan_root.as_ref().expect("should have scan root");
            assert!(
                !root.children.is_empty(),
                "root should have children after scan"
            );

            let child_names: Vec<&str> = root.children.iter().map(|c| c.name.as_str()).collect();
            assert!(
                child_names.contains(&"Users"),
                "should find Users folder, got: {child_names:?}"
            );
            assert!(
                child_names.contains(&"Windows"),
                "should find Windows folder, got: {child_names:?}"
            );

            // Verify sizes propagated
            assert!(
                root.current_size > 0,
                "root size should be > 0, got: {}",
                root.current_size
            );

            let users = root.children.iter().find(|c| c.name == "Users").unwrap();
            assert_eq!(users.current_size, 150, "Users = docs/readme.txt(100) + file.txt(50)");
            assert!(
                !users.children.is_empty(),
                "Users should have its own children"
            );
        });

        // Tree view shows collapsed root (not auto-expanded)
        let tree_view = view.read_with(cx, |v, _| v.tree_view.clone());
        tree_view.read_with(cx, |v, _| {
            assert_eq!(
                v.nodes.len(),
                1,
                "tree should show only collapsed root, got {} nodes",
                v.nodes.len()
            );
        });
    }

    #[gpui::test]
    fn expand_folder_shows_only_immediate_children(cx: &mut gpui::TestAppContext) {
        cx.update(|app| gpui_component::init(app));

        let test_dir = make_test_dir();
        let test_path = test_dir.path().to_path_buf();

        let db = persistence::open_in_memory().unwrap();

        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut app = AppView::new_with_db(db, window, cx);
            app.scan_root_override = Some(test_path.clone());
            app
        });

        // 1. Select drive and run a scan to populate the tree
        view.update(cx, |v, cx| {
            v.on_drive_selected("C:".to_string(), cx);
            v.start_scan(cx);
        });
        std::thread::sleep(std::time::Duration::from_millis(200));
        cx.executor().advance_clock(std::time::Duration::from_millis(20));
        cx.run_until_parked();

        // Scan should be done — root stays collapsed (not auto-expanded)
        let tree_view = view.read_with(cx, |v, _| v.tree_view.clone());

        // 2. Tree should show only the collapsed root
        tree_view.read_with(cx, |v, _| {
            assert_eq!(v.nodes.len(), 1, "should show only the collapsed root");
            assert!(!v.nodes[0].expanded, "root should be collapsed");
        });

        // 3. Expand root → only immediate children (Users, Windows)
        view.update(cx, |v, cx| {
            v.on_toggle_expand(test_path.clone(), cx);
        });
        cx.run_until_parked();

        tree_view.read_with(cx, |v, _| {
            // root + Users + Windows = 3
            assert_eq!(
                v.nodes.len(),
                3,
                "root expanded should show root + 2 children, got: {:?}",
                v.nodes.iter().map(|n| &n.fs_node.name).collect::<Vec<_>>()
            );
            assert!(v.nodes[0].expanded, "root should be expanded");

            // Children at depth 1, collapsed
            for node in &v.nodes[1..] {
                assert_eq!(node.depth, 1);
                assert!(!node.expanded);
            }

            let child_names: Vec<&str> =
                v.nodes[1..].iter().map(|n| n.fs_node.name.as_str()).collect();
            assert!(child_names.contains(&"Users"), "got: {child_names:?}");
            assert!(child_names.contains(&"Windows"), "got: {child_names:?}");
        });

        // 4. Expand Users → only its immediate children (docs, file.txt)
        //    NOT docs/readme.txt (docs is still collapsed)
        let users_path = test_path.join("Users");
        view.update(cx, |v, cx| {
            v.on_toggle_expand(users_path, cx);
        });
        cx.run_until_parked();

        tree_view.read_with(cx, |v, _| {
            // root + Users(expanded) + docs + file.txt + Windows = 5
            assert_eq!(
                v.nodes.len(),
                5,
                "expanding Users should add docs + file.txt, got: {:?}",
                v.nodes.iter().map(|n| (&n.fs_node.name, n.depth)).collect::<Vec<_>>()
            );

            let depth2: Vec<&str> = v
                .nodes
                .iter()
                .filter(|n| n.depth == 2)
                .map(|n| n.fs_node.name.as_str())
                .collect();
            assert!(depth2.contains(&"docs"), "got: {depth2:?}");
            assert!(depth2.contains(&"file.txt"), "got: {depth2:?}");

            // readme.txt must NOT be visible — docs is collapsed
            let all_names: Vec<&str> =
                v.nodes.iter().map(|n| n.fs_node.name.as_str()).collect();
            assert!(
                !all_names.contains(&"readme.txt"),
                "readme.txt should not be visible — docs is still collapsed"
            );
        });
    }

    #[gpui::test]
    fn scan_cancel_stops_workers(cx: &mut gpui::TestAppContext) {
        cx.update(|app| gpui_component::init(app));

        let test_dir = make_test_dir();
        let test_path = test_dir.path().to_path_buf();

        let db = persistence::open_in_memory().unwrap();

        let (view, cx) = cx.add_window_view(|window, cx| {
            let mut app = AppView::new_with_db(db, window, cx);
            app.scan_root_override = Some(test_path);
            app
        });

        // Select drive and start scan
        view.update(cx, |v, cx| {
            v.on_drive_selected("C:".to_string(), cx);
            v.start_scan(cx);
        });

        // Cancel immediately
        view.update(cx, |v, cx| {
            v.start_scan(cx); // second call = cancel
            let _ = cx; // suppress warning
        });

        // Let scanner finish, then process all channel messages
        std::thread::sleep(std::time::Duration::from_millis(200));
        cx.executor().advance_clock(std::time::Duration::from_millis(20));
        cx.run_until_parked();

        view.read_with(cx, |v, _| {
            assert!(!v.scanning, "should no longer be scanning after cancel");
            assert!(!v.scan_completed, "scan_completed should be false after cancel");
            assert_eq!(v.scan_status.as_ref(), "Scan cancelled");
        });
    }
}
