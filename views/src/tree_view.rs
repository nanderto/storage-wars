//! Hierarchical file tree view with columns, chevrons, icons, and progress bars.

use gpui::{
    div, px, Context, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
    ViewContext,
};

use crate::theme::{Palette, SizeChange, TREE_INDENT_PX, TREE_ROW_HEIGHT_PX};

/// A single node in the file tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub id: u64,
    pub name: String,
    pub depth: usize,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub has_children: bool,
    /// Size in bytes for the current scan.
    pub size_bytes: u64,
    /// Size in bytes from the previous scan, if available.
    pub prev_size_bytes: Option<u64>,
    /// Size of the parent directory (used to compute % Parent).
    pub parent_size_bytes: u64,
    pub file_count: u64,
    pub folder_count: u64,
    /// Last modified timestamp as a formatted string.
    pub modified: String,
}

impl TreeNode {
    /// Percentage of the parent directory's size occupied by this node.
    pub fn percent_of_parent(&self) -> f64 {
        if self.parent_size_bytes == 0 {
            return 0.0;
        }
        (self.size_bytes as f64 / self.parent_size_bytes as f64) * 100.0
    }

    /// Percentage change compared to the previous scan.
    pub fn percent_change(&self) -> Option<f64> {
        let prev = self.prev_size_bytes?;
        if prev == 0 {
            return None;
        }
        Some(((self.size_bytes as f64 - prev as f64) / prev as f64) * 100.0)
    }

    /// Size change classification for coloring.
    pub fn size_change(&self) -> SizeChange {
        SizeChange::classify(self.prev_size_bytes, Some(self.size_bytes))
    }
}

/// Column definitions for the tree view header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Column {
    Name,
    PercentParent,
    Size,
    PrevSize,
    PercentPrev,
    Files,
    Folders,
    Modified,
}

impl Column {
    pub fn label(self) -> &'static str {
        match self {
            Column::Name => "Name",
            Column::PercentParent => "% Parent",
            Column::Size => "Size",
            Column::PrevSize => "Prev Size",
            Column::PercentPrev => "% Prev",
            Column::Files => "Files",
            Column::Folders => "Folders",
            Column::Modified => "Modified",
        }
    }

    pub fn min_width_px(self) -> f32 {
        match self {
            Column::Name => 200.0,
            Column::PercentParent => 80.0,
            Column::Size => 90.0,
            Column::PrevSize => 90.0,
            Column::PercentPrev => 80.0,
            Column::Files => 70.0,
            Column::Folders => 70.0,
            Column::Modified => 130.0,
        }
    }

    pub const ALL: &'static [Column] = &[
        Column::Name,
        Column::PercentParent,
        Column::Size,
        Column::PrevSize,
        Column::PercentPrev,
        Column::Files,
        Column::Folders,
        Column::Modified,
    ];
}

/// Hierarchical file list view.
pub struct TreeView {
    focus_handle: FocusHandle,
    nodes: Vec<TreeNode>,
    selected_id: Option<u64>,
    sort_column: Column,
    sort_ascending: bool,
}

impl TreeView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            nodes: Vec::new(),
            selected_id: None,
            sort_column: Column::Size,
            sort_ascending: false,
        }
    }

    /// Replaces the node list.
    pub fn set_nodes(&mut self, nodes: Vec<TreeNode>, cx: &mut ViewContext<Self>) {
        self.nodes = nodes;
        cx.notify();
    }

    fn select_node(&mut self, id: u64, cx: &mut ViewContext<Self>) {
        self.selected_id = Some(id);
        cx.notify();
    }

    fn toggle_sort(&mut self, column: Column, cx: &mut ViewContext<Self>) {
        if self.sort_column == column {
            self.sort_ascending = !self.sort_ascending;
        } else {
            self.sort_column = column;
            self.sort_ascending = false;
        }
        cx.notify();
    }

    /// Renders the column header row.
    fn render_header(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let columns: Vec<_> = Column::ALL
            .iter()
            .map(|&col| {
                let is_sorted = self.sort_column == col;
                let arrow = if is_sorted {
                    if self.sort_ascending { " ▲" } else { " ▼" }
                } else {
                    ""
                };
                div()
                    .flex()
                    .items_center()
                    .min_w(px(col.min_width_px()))
                    .h_full()
                    .px(px(6.0))
                    .border_r_1()
                    .border_color(Palette::border())
                    .cursor_pointer()
                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, cx| {
                        this.toggle_sort(col, cx);
                    }))
                    .child(
                        div()
                            .text_color(if is_sorted {
                                Palette::accent()
                            } else {
                                Palette::text_secondary()
                            })
                            .text_xs()
                            .child(format!("{}{}", col.label(), arrow)),
                    )
            })
            .collect();

        div()
            .flex()
            .flex_row()
            .w_full()
            .h(px(TREE_ROW_HEIGHT_PX))
            .bg(Palette::surface())
            .border_b_1()
            .border_color(Palette::border())
            .children(columns)
    }

    /// Renders a single tree row.
    fn render_row(&self, node: &TreeNode, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_selected = self.selected_id == Some(node.id);
        let indent = node.depth as f32 * TREE_INDENT_PX;
        let size_change = node.size_change();
        let bar_color = size_change.color();
        let pct_parent = node.percent_of_parent().clamp(0.0, 100.0);
        let node_id = node.id;

        // ── Name cell ──────────────────────────────────────────────────────
        let chevron = if node.has_children {
            if node.is_expanded { "▾ " } else { "▸ " }
        } else {
            "  "
        };
        let icon = if node.is_dir { "📁 " } else { "📄 " };

        let name_cell = div()
            .flex()
            .flex_row()
            .items_center()
            .min_w(px(Column::Name.min_width_px()))
            .h_full()
            .px(px(6.0))
            .border_r_1()
            .border_color(Palette::border())
            .child(div().w(px(indent)))
            .child(
                div()
                    .text_color(Palette::text_secondary())
                    .text_xs()
                    .child(chevron),
            )
            .child(
                div()
                    .text_color(Palette::text_secondary())
                    .text_xs()
                    .child(icon),
            )
            .child(
                div()
                    .text_color(Palette::text_primary())
                    .text_sm()
                    .overflow_hidden()
                    .child(node.name.clone()),
            );

        // ── % Parent cell (progress bar) ───────────────────────────────────
        let pct_parent_cell = div()
            .flex()
            .flex_row()
            .items_center()
            .min_w(px(Column::PercentParent.min_width_px()))
            .h_full()
            .px(px(6.0))
            .border_r_1()
            .border_color(Palette::border())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .w_full()
                    .h(px(10.0))
                    .bg(Palette::progress_track())
                    .rounded(px(2.0))
                    .overflow_hidden()
                    .child(
                        div()
                            .h_full()
                            .w(gpui::relative(pct_parent as f32 / 100.0))
                            .bg(bar_color)
                            .rounded(px(2.0)),
                    ),
            );

        // ── Numeric cells ──────────────────────────────────────────────────
        let size_cell = self.numeric_cell(
            &crate::drive_selector::format_bytes(node.size_bytes),
            Column::Size,
        );
        let prev_size_cell = self.numeric_cell(
            &node
                .prev_size_bytes
                .map(crate::drive_selector::format_bytes)
                .unwrap_or_else(|| "—".to_string()),
            Column::PrevSize,
        );
        let pct_prev_cell = self.numeric_cell(
            &node
                .percent_change()
                .map(|p| format!("{:+.1}%", p))
                .unwrap_or_else(|| "—".to_string()),
            Column::PercentPrev,
        );
        let files_cell = self.numeric_cell(&node.file_count.to_string(), Column::Files);
        let folders_cell = self.numeric_cell(&node.folder_count.to_string(), Column::Folders);
        let modified_cell = self.numeric_cell(&node.modified, Column::Modified);

        div()
            .flex()
            .flex_row()
            .w_full()
            .h(px(TREE_ROW_HEIGHT_PX))
            .bg(if is_selected {
                Palette::selection()
            } else {
                Palette::background()
            })
            .hover(|s| s.bg(Palette::surface()))
            .border_b_1()
            .border_color(Palette::border())
            .cursor_pointer()
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, cx| {
                this.select_node(node_id, cx);
            }))
            .child(name_cell)
            .child(pct_parent_cell)
            .child(size_cell)
            .child(prev_size_cell)
            .child(pct_prev_cell)
            .child(files_cell)
            .child(folders_cell)
            .child(modified_cell)
    }

    fn numeric_cell(&self, text: &str, column: Column) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_end()
            .min_w(px(column.min_width_px()))
            .h_full()
            .px(px(6.0))
            .border_r_1()
            .border_color(Palette::border())
            .child(
                div()
                    .text_color(Palette::text_secondary())
                    .text_sm()
                    .child(text.to_string()),
            )
    }
}

impl Focusable for TreeView {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TreeView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let rows: Vec<_> = self
            .nodes
            .clone()
            .iter()
            .map(|node| self.render_row(node, cx))
            .collect();

        div()
            .flex()
            .flex_col()
            .size_full()
            .overflow_hidden()
            .child(self.render_header(cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(rows),
            )
    }
}