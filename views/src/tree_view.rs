use gpui::{
    div, px, App, Context, Element, EventEmitter, Focusable, FocusHandle, InteractiveElement,
    IntoElement, ParentElement, Render, SharedString, Styled, Window,
};
use uuid::Uuid;

use crate::theme::current_theme;
use crate::types::{SizeChange, TreeNode};

const INDENT_PX: f32 = 16.0;
const ROW_HEIGHT: f32 = 28.0;

/// Events emitted by the tree view.
pub enum TreeViewEvent {
    NodeSelected(Uuid),
    NodeExpanded(Uuid),
    NodeCollapsed(Uuid),
}

impl EventEmitter<TreeViewEvent> for TreeView {}

/// Renders a hierarchical file list with columns.
pub struct TreeView {
    focus_handle: FocusHandle,
    root: Option<TreeNode>,
    selected_id: Option<Uuid>,
    parent_size: u64,
}

impl TreeView {
    pub fn new(
        root: Option<TreeNode>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let parent_size = root.as_ref().map(|r| r.size_bytes).unwrap_or(0);
        Self {
            focus_handle: cx.focus_handle(),
            root,
            selected_id: None,
            parent_size,
        }
    }

    pub fn set_root(&mut self, root: Option<TreeNode>, cx: &mut Context<Self>) {
        self.parent_size = root.as_ref().map(|r| r.size_bytes).unwrap_or(0);
        self.root = root;
        cx.notify();
    }

    /// Flatten the tree into a list of visible rows.
    fn visible_rows(node: &TreeNode) -> Vec<FlatRow> {
        let mut rows = Vec::new();
        Self::collect_rows(node, &mut rows);
        rows
    }

    fn collect_rows(node: &TreeNode, out: &mut Vec<FlatRow>) {
        out.push(FlatRow {
            id: node.id,
            name: node.name.clone(),
            depth: node.depth,
            is_dir: node.is_dir,
            is_expanded: node.is_expanded,
            size_bytes: node.size_bytes,
            prev_size_bytes: node.prev_size_bytes,
            file_count: node.file_count,
            folder_count: node.folder_count,
            modified_at: node
                .modified_at
                .map(|d| d.format("%Y-%m-%d").to_string()),
            size_change: node.size_change(),
        });
        if node.is_expanded {
            for child in &node.children {
                Self::collect_rows(child, out);
            }
        }
    }

    fn format_size(bytes: u64) -> String {
        let gb = bytes as f64 / 1_073_741_824.0;
        if gb >= 1.0 {
            return format!("{gb:.2} GB");
        }
        let mb = bytes as f64 / 1_048_576.0;
        if mb >= 1.0 {
            return format!("{mb:.1} MB");
        }
        let kb = bytes as f64 / 1_024.0;
        format!("{kb:.0} KB")
    }
}

#[derive(Clone)]
struct FlatRow {
    id: Uuid,
    name: String,
    depth: usize,
    is_dir: bool,
    is_expanded: bool,
    size_bytes: u64,
    prev_size_bytes: Option<u64>,
    file_count: u64,
    folder_count: u64,
    modified_at: Option<String>,
    size_change: SizeChange,
}

impl Focusable for TreeView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TreeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = current_theme();
        let is_focused = self.focus_handle.is_focused(cx);
        let parent_size = self.parent_size;
        let selected_id = self.selected_id;

        let rows: Vec<FlatRow> = self
            .root
            .as_ref()
            .map(Self::visible_rows)
            .unwrap_or_default();

        let border_color = if is_focused {
            theme.border_focused
        } else {
            theme.border
        };

        div()
            .id("tree-view")
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .border_1()
            .border_color(border_color)
            .overflow_hidden()
            // Column headers
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .h(px(28.0))
                    .bg(theme.column_header_bg)
                    .border_b_1()
                    .border_color(theme.border)
                    .child(col_header("Name", px(280.0), &theme))
                    .child(col_header("% Parent", px(80.0), &theme))
                    .child(col_header("Size", px(90.0), &theme))
                    .child(col_header("Prev Size", px(90.0), &theme))
                    .child(col_header("% Prev", px(80.0), &theme))
                    .child(col_header("Files", px(70.0), &theme))
                    .child(col_header("Folders", px(70.0), &theme))
                    .child(col_header("Modified", px(100.0), &theme)),
            )
            // Rows
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(rows.into_iter().enumerate().map(|(i, row)| {
                        let row_id = row.id;
                        let is_selected = selected_id == Some(row_id);
                        let indent = row.depth as f32 * INDENT_PX;
                        let pct_parent = if parent_size > 0 {
                            row.size_bytes as f32 / parent_size as f32
                        } else {
                            0.0
                        };
                        let pct_prev = row.prev_size_bytes.map(|prev| {
                            if prev == 0 {
                                100.0f32
                            } else {
                                (row.size_bytes as f64 / prev as f64 * 100.0 - 100.0) as f32
                            }
                        });

                        let row_bg = if is_selected {
                            theme.selection_bg
                        } else if i % 2 == 0 {
                            theme.row_even
                        } else {
                            theme.row_odd
                        };

                        let change_color = match row.size_change {
                            SizeChange::Increased => theme.size_increased,
                            SizeChange::Decreased => theme.size_decreased,
                            SizeChange::New => theme.size_new,
                            SizeChange::Unchanged => theme.size_unchanged,
                        };

                        let size_str = TreeView::format_size(row.size_bytes);
                        let prev_str = row
                            .prev_size_bytes
                            .map(TreeView::format_size)
                            .unwrap_or_else(|| "—".to_string());
                        let pct_prev_str = pct_prev
                            .map(|p| format!("{p:+.1}%"))
                            .unwrap_or_else(|| "—".to_string());
                        let modified_str = row
                            .modified_at
                            .clone()
                            .unwrap_or_else(|| "—".to_string());

                        div()
                            .id(("tree-row", i))
                            .flex()
                            .flex_row()
                            .items_center()
                            .h(px(ROW_HEIGHT))
                            .bg(row_bg)
                            .hover(|s| s.bg(theme.row_hover))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                this.selected_id = Some(row_id);
                                cx.emit(TreeViewEvent::NodeSelected(row_id));
                                cx.notify();
                            }))
                            // Name column with indent + chevron + icon
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .w(px(280.0))
                                    .pl(px(8.0 + indent))
                                    .overflow_hidden()
                                    .child(
                                        div()
                                            .w(px(16.0))
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(if row.is_dir {
                                                if row.is_expanded { "▾" } else { "▸" }
                                            } else {
                                                " "
                                            }),
                                    )
                                    .child(
                                        div()
                                            .w(px(16.0))
                                            .text_xs()
                                            .child(if row.is_dir { "📁" } else { "📄" }),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.text_primary)
                                            .overflow_hidden()
                                            .child(SharedString::from(row.name.clone())),
                                    ),
                            )
                            // % Parent column with progress bar
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .justify_center()
                                    .w(px(80.0))
                                    .px(px(6.0))
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .child(SharedString::from(format!(
                                                "{:.1}%",
                                                pct_parent * 100.0
                                            ))),
                                    )
                                    .child(progress_bar(pct_parent, change_color, &theme)),
                            )
                            // Size
                            .child(cell(size_str, px(90.0), theme.text_secondary))
                            // Prev Size
                            .child(cell(prev_str, px(90.0), theme.text_muted))
                            // % Prev
                            .child(
                                div()
                                    .w(px(80.0))
                                    .px(px(6.0))
                                    .text_xs()
                                    .text_color(change_color)
                                    .child(SharedString::from(pct_prev_str)),
                            )
                            // Files
                            .child(cell(
                                format_count(row.file_count),
                                px(70.0),
                                theme.text_muted,
                            ))
                            // Folders
                            .child(cell(
                                format_count(row.folder_count),
                                px(70.0),
                                theme.text_muted,
                            ))
                            // Modified
                            .child(cell(modified_str, px(100.0), theme.text_muted))
                    })),
            )
    }
}

fn col_header(label: &'static str, width: gpui::Pixels, theme: &crate::theme::Theme) -> impl IntoElement {
    div()
        .w(width)
        .h_full()
        .flex()
        .items_center()
        .px(px(6.0))
        .border_r_1()
        .border_color(theme.border)
        .child(
            div()
                .text_xs()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(theme.text_secondary)
                .child(label),
        )
}

fn cell(content: String, width: gpui::Pixels, color: gpui::Hsla) -> impl IntoElement {
    div()
        .w(width)
        .px(px(6.0))
        .text_xs()
        .text_color(color)
        .overflow_hidden()
        .child(SharedString::from(content))
}

fn progress_bar(fraction: f32, fill_color: gpui::Hsla, theme: &crate::theme::Theme) -> impl IntoElement {
    let clamped = fraction.clamp(0.0, 1.0);
    div()
        .w_full()
        .h(px(4.0))
        .rounded_full()
        .bg(theme.progress_track)
        .child(
            div()
                .h_full()
                .w(gpui::relative(clamped))
                .rounded_full()
                .bg(fill_color),
        )
}

fn format_count(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}