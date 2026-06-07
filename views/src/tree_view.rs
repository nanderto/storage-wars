use gpui::{
    div, px, AppContext, Div, EventEmitter, FocusHandle, FocusableView, IntoElement,
    ParentElement, Render, SharedString, Styled, View, ViewContext, VisualContext, WindowContext,
};
use std::collections::HashSet;
use uuid::Uuid;

use crate::types::{SizeChange, TreeNode, format_bytes, format_percent};
use crate::ui_helpers::{colors, font_size, spacing, progress_bar};

/// Column definitions for the tree view
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeColumn {
    Name,
    PercentParent,
    Size,
    PrevSize,
    PercentPrev,
    Files,
    Folders,
    Modified,
}

impl TreeColumn {
    pub fn label(&self) -> &'static str {
        match self {
            TreeColumn::Name => "Name",
            TreeColumn::PercentParent => "% Parent",
            TreeColumn::Size => "Size",
            TreeColumn::PrevSize => "Prev Size",
            TreeColumn::PercentPrev => "% Prev",
            TreeColumn::Files => "Files",
            TreeColumn::Folders => "Folders",
            TreeColumn::Modified => "Modified",
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            TreeColumn::Name => 300.0,
            TreeColumn::PercentParent => 100.0,
            TreeColumn::Size => 90.0,
            TreeColumn::PrevSize => 90.0,
            TreeColumn::PercentPrev => 80.0,
            TreeColumn::Files => 70.0,
            TreeColumn::Folders => 70.0,
            TreeColumn::Modified => 140.0,
        }
    }
}

/// A flat row representation for rendering
#[derive(Debug, Clone)]
pub struct TreeRow {
    pub node_id: Uuid,
    pub name: String,
    pub path: String,
    pub depth: usize,
    pub is_directory: bool,
    pub is_expanded: bool,
    pub has_children: bool,
    pub size_bytes: u64,
    pub prev_size_bytes: Option<u64>,
    pub parent_size_bytes: Option<u64>,
    pub file_count: u64,
    pub folder_count: u64,
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub size_change: SizeChange,
}

impl TreeRow {
    pub fn percent_of_parent(&self) -> Option<f64> {
        self.parent_size_bytes.and_then(|parent| {
            if parent == 0 {
                None
            } else {
                Some((self.size_bytes as f64 / parent as f64) * 100.0)
            }
        })
    }

    pub fn percent_of_prev(&self) -> Option<f64> {
        self.prev_size_bytes.and_then(|prev| {
            if prev == 0 {
                None
            } else {
                Some((self.size_bytes as f64 / prev as f64) * 100.0)
            }
        })
    }
}

/// Events emitted by TreeView
#[derive(Debug, Clone)]
pub enum TreeViewEvent {
    NodeToggled(Uuid),
    NodeSelected(Uuid),
}

/// Hierarchical file tree view with columns
pub struct TreeView {
    rows: Vec<TreeRow>,
    selected_id: Option<Uuid>,
    expanded_ids: HashSet<Uuid>,
    focus_handle: FocusHandle,
    scroll_offset: f32,
}

impl TreeView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            rows: Vec::new(),
            selected_id: None,
            expanded_ids: HashSet::new(),
            focus_handle: cx.focus_handle(),
            scroll_offset: 0.0,
        }
    }

    pub fn set_rows(&mut self, rows: Vec<TreeRow>, cx: &mut ViewContext<Self>) {
        self.rows = rows;
        cx.notify();
    }

    pub fn clear(&mut self, cx: &mut ViewContext<Self>) {
        self.rows.clear();
        self.selected_id = None;
        self.expanded_ids.clear();
        cx.notify();
    }

    fn row_height() -> f32 {
        28.0
    }

    fn render_header(&self) -> Div {
        let columns = [
            TreeColumn::Name,
            TreeColumn::PercentParent,
            TreeColumn::Size,
            TreeColumn::PrevSize,
            TreeColumn::PercentPrev,
            TreeColumn::Files,
            TreeColumn::Folders,
            TreeColumn::Modified,
        ];

        let mut header = div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(28.0))
            .bg(colors::surface())
            .border_b_1()
            .border_color(colors::border())
            .px(spacing::sm());

        for col in &columns {
            header = header.child(
                div()
                    .w(px(col.width()))
                    .flex_shrink_0()
                    .text_color(colors::subtext())
                    .text_size(font_size::xs())
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(col.label()),
            );
        }

        header
    }

    fn render_row(&self, row: &TreeRow, cx: &mut ViewContext<Self>) -> Div {
        let is_selected = self.selected_id == Some(row.node_id);
        let indent = row.depth as f32 * spacing::tree_indent().0;
        let node_id = row.node_id;

        // Name cell with chevron and icon
        let chevron = if row.is_directory && row.has_children {
            if row.is_expanded { "▾ " } else { "▸ " }
        } else if row.is_directory {
            "  "
        } else {
            "  "
        };

        let icon = if row.is_directory { "📁" } else { "📄" };

        let name_cell = div()
            .flex()
            .flex_row()
            .items_center()
            .w(px(TreeColumn::Name.width()))
            .flex_shrink_0()
            .pl(px(indent))
            .child(
                div()
                    .text_color(colors::accent())
                    .text_size(font_size::sm())
                    .cursor_pointer()
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(move |this, _, cx| {
                            cx.emit(TreeViewEvent::NodeToggled(node_id));
                        }),
                    )
                    .child(chevron),
            )
            .child(
                div()
                    .text_size(font_size::sm())
                    .mr(spacing::xs())
                    .child(icon),
            )
            .child(
                div()
                    .text_color(colors::text())
                    .text_size(font_size::sm())
                    .overflow_hidden()
                    .child(SharedString::from(row.name.clone())),
            );

        // Percent of parent cell with progress bar
        let pct_parent_cell = div()
            .w(px(TreeColumn::PercentParent.width()))
            .flex_shrink_0()
            .flex()
            .flex_col()
            .justify_center()
            .gap(px(2.0))
            .child(match row.percent_of_parent() {
                Some(pct) => div()
                    .text_color(colors::for_size_change(row.size_change))
                    .text_size(font_size::xs())
                    .child(SharedString::from(format_percent(pct))),
                None => div()
                    .text_color(colors::muted())
                    .text_size(font_size::xs())
                    .child("—"),
            })
            .child(progress_bar(
                row.percent_of_parent().map(|p| (p / 100.0) as f32).unwrap_or(0.0),
                row.size_change,
            ));

        // Size cell
        let size_cell = div()
            .w(px(TreeColumn::Size.width()))
            .flex_shrink_0()
            .text_color(colors::text())
            .text_size(font_size::sm())
            .child(SharedString::from(format_bytes(row.size_bytes)));

        // Prev size cell
        let prev_size_cell = div()
            .w(px(TreeColumn::PrevSize.width()))
            .flex_shrink_0()
            .text_color(colors::subtext())
            .text_size(font_size::sm())
            .child(match row.prev_size_bytes {
                Some(prev) => SharedString::from(format_bytes(prev)),
                None => "—".into(),
            });

        // Percent prev cell
        let pct_prev_cell = div()
            .w(px(TreeColumn::PercentPrev.width()))
            .flex_shrink_0()
            .text_color(colors::for_size_change(row.size_change))
            .text_size(font_size::sm())
            .child(match row.percent_of_prev() {
                Some(pct) => SharedString::from(format_percent(pct)),
                None => "—".into(),
            });

        // Files cell
        let files_cell = div()
            .w(px(TreeColumn::Files.width()))
            .flex_shrink_0()
            .text_color(colors::subtext())
            .text_size(font_size::sm())
            .child(SharedString::from(row.file_count.to_string()));

        // Folders cell
        let folders_cell = div()
            .w(px(TreeColumn::Folders.width()))
            .flex_shrink_0()
            .text_color(colors::subtext())
            .text_size(font_size::sm())
            .child(SharedString::from(row.folder_count.to_string()));

        // Modified cell
        let modified_cell = div()
            .w(px(TreeColumn::Modified.width()))
            .flex_shrink_0()
            .text_color(colors::muted())
            .text_size(font_size::xs())
            .child(match &row.modified_at {
                Some(dt) => SharedString::from(dt.format("%Y-%m-%d %H:%M").to_string()),
                None => "—".into(),
            });

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(Self::row_height()))
            .px(spacing::sm())
            .bg(if is_selected {
                colors::overlay()
            } else {
                colors::background()
            })
            .hover(|s| s.bg(colors::surface()))
            .cursor_pointer()
            .border_b_1()
            .border_color(colors::border())
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(move |this, _, cx| {
                    this.selected_id = Some(node_id);
                    cx.emit(TreeViewEvent::NodeSelected(node_id));
                    cx.notify();
                }),
            )
            .child(name_cell)
            .child(pct_parent_cell)
            .child(size_cell)
            .child(prev_size_cell)
            .child(pct_prev_cell)
            .child(files_cell)
            .child(folders_cell)
            .child(modified_cell)
    }

    fn render_empty_state(&self) -> Div {
        div()
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(spacing::sm())
                    .child(
                        div()
                            .text_size(px(32.0))
                            .child("🔍"),
                    )
                    .child(
                        div()
                            .text_color(colors::muted())
                            .text_size(font_size::md())
                            .child("No scan data available"),
                    )
                    .child(
                        div()
                            .text_color(colors::muted())
                            .text_size(font_size::sm())
                            .child("Select a drive and start a scan"),
                    ),
            )
    }
}

impl EventEmitter<TreeViewEvent> for TreeView {}

impl FocusableView for TreeView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TreeView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let rows = self.rows.clone();

        let mut container = div()
            .flex()
            .flex_col()
            .w_full()
            .flex_1()
            .overflow_hidden()
            .track_focus(&self.focus_handle);

        container = container.child(self.render_header());

        if rows.is_empty() {
            container = container.child(self.render_empty_state());
        } else {
            let mut scroll_area = div()
                .flex()
                .flex_col()
                .w_full()
                .flex_1()
                .overflow_y_scroll();

            for row in &rows {
                scroll_area = scroll_area.child(self.render_row(row, cx));
            }

            container = container.child(scroll_area);
        }

        container
    }
}