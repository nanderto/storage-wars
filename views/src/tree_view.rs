//! Hierarchical file/folder tree view with sortable columns.

use gpui::{
    div, prelude::*, px, Context, Element, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};

use crate::theme::{
    SizeChange, COLOR_ACCENT, COLOR_BACKGROUND, COLOR_BORDER, COLOR_SURFACE, COLOR_TEXT_PRIMARY,
    COLOR_TEXT_SECONDARY, TREE_INDENT_PX,
};

/// Column identifiers for the tree view.
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
    /// Human-readable header label.
    pub fn header(self) -> &'static str {
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

    /// Minimum column width in pixels.
    pub fn min_width_px(self) -> f32 {
        match self {
            Column::Name => 200.0,
            Column::PercentParent => 80.0,
            Column::Size => 80.0,
            Column::PrevSize => 80.0,
            Column::PercentPrev => 72.0,
            Column::Files => 64.0,
            Column::Folders => 64.0,
            Column::Modified => 120.0,
        }
    }
}

/// A single node in the tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Display name of the file or directory.
    pub name: String,
    /// Nesting depth (0 = root).
    pub depth: usize,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Whether this directory node is currently expanded.
    pub is_expanded: bool,
    /// Current size in bytes.
    pub size_bytes: u64,
    /// Size from the previous scan, if available.
    pub prev_size_bytes: Option<u64>,
    /// Number of files under this node.
    pub file_count: u64,
    /// Number of sub-folders under this node.
    pub folder_count: u64,
    /// Last-modified timestamp string, if available.
    pub modified: Option<String>,
    /// Percentage of the parent node's total size (0–100).
    pub percent_parent: f32,
    /// Direction of size change relative to the previous scan.
    pub size_change: SizeChange,
    /// Direct children (rendered recursively when expanded).
    pub children: Vec<TreeNode>,
}

/// Hierarchical file list view.
pub struct TreeView {
    focus_handle: FocusHandle,
    nodes: Vec<TreeNode>,
    sort_column: Column,
    sort_ascending: bool,
}

impl TreeView {
    /// Constructs the view within a GPUI context.
    pub fn build(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            nodes: Vec::new(),
            sort_column: Column::Size,
            sort_ascending: false,
        }
    }

    /// Replaces the root node list.
    pub fn set_nodes(&mut self, nodes: Vec<TreeNode>) {
        self.nodes = nodes;
    }
}

impl Focusable for TreeView {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TreeView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .bg(COLOR_BACKGROUND)
            .child(self.render_header())
            .child(self.render_body())
    }
}

impl TreeView {
    fn render_header(&self) -> impl IntoElement {
        use Column::*;
        let columns = [
            Name,
            PercentParent,
            Size,
            PrevSize,
            PercentPrev,
            Files,
            Folders,
            Modified,
        ];

        let mut row = div()
            .flex()
            .flex_row()
            .w_full()
            .h(px(28.0))
            .bg(COLOR_SURFACE)
            .border_b_1()
            .border_color(COLOR_BORDER);

        for col in columns {
            let is_sorted = col == self.sort_column;
            let indicator = if is_sorted {
                if self.sort_ascending { " ▲" } else { " ▼" }
            } else {
                ""
            };

            row = row.child(
                div()
                    .flex()
                    .items_center()
                    .px(px(8.0))
                    .min_w(px(col.min_width_px()))
                    .text_color(if is_sorted {
                        COLOR_ACCENT
                    } else {
                        COLOR_TEXT_SECONDARY
                    })
                    .child(format!("{}{}", col.header(), indicator)),
            );
        }

        row
    }

    fn render_body(&self) -> impl IntoElement {
        let mut col = div()
            .flex()
            .flex_col()
            .flex_1()
            .overflow_hidden()
            .bg(COLOR_BACKGROUND);

        if self.nodes.is_empty() {
            col = col.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_1()
                    .text_color(COLOR_TEXT_SECONDARY)
                    .child("No scan data — select a drive and press Scan"),
            );
        } else {
            for node in &self.nodes {
                col = col.child(render_node(node));
            }
        }

        col
    }
}

fn render_node(node: &TreeNode) -> impl IntoElement {
    let indent = node.depth as f32 * TREE_INDENT_PX;

    let chevron = if node.is_dir {
        if node.is_expanded { "▾ " } else { "▸ " }
    } else {
        "  "
    };

    let icon = if node.is_dir { "📁 " } else { "📄 " };

    let size_label = format_bytes(node.size_bytes);
    let prev_label = node
        .prev_size_bytes
        .map(format_bytes)
        .unwrap_or_default();
    let pct_prev = node.prev_size_bytes.map(|prev| {
        if prev == 0 {
            0.0_f32
        } else {
            (node.size_bytes as f32 - prev as f32) / prev as f32 * 100.0
        }
    });

    let bar_color = node.size_change.color();
    let bar_width = (node.percent_parent.clamp(0.0, 100.0) / 100.0) * 80.0;

    div()
        .flex()
        .flex_row()
        .items_center()
        .w_full()
        .h(px(22.0))
        .border_b_1()
        .border_color(COLOR_BORDER)
        // Name column with indentation
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .min_w(px(200.0))
                .pl(px(indent + 8.0))
                .text_color(COLOR_TEXT_PRIMARY)
                .child(format!("{}{}{}", chevron, icon, node.name)),
        )
        // % Parent with progress bar overlay
        .child(
            div()
                .relative()
                .min_w(px(80.0))
                .h_full()
                .flex()
                .items_center()
                .px(px(8.0))
                .child(
                    div()
                        .absolute()
                        .left(px(0.0))
                        .top(px(0.0))
                        .h_full()
                        .w(px(bar_width))
                        .bg(bar_color)
                        .opacity(0.25),
                )
                .child(
                    div()
                        .text_color(COLOR_TEXT_PRIMARY)
                        .child(format!("{:.1}%", node.percent_parent)),
                ),
        )
        // Size
        .child(
            div()
                .min_w(px(80.0))
                .px(px(8.0))
                .text_color(COLOR_TEXT_PRIMARY)
                .child(size_label),
        )
        // Prev Size
        .child(
            div()
                .min_w(px(80.0))
                .px(px(8.0))
                .text_color(COLOR_TEXT_SECONDARY)
                .child(prev_label),
        )
        // % Prev
        .child(
            div()
                .min_w(px(72.0))
                .px(px(8.0))
                .text_color(bar_color)
                .child(
                    pct_prev
                        .map(|p| format!("{:+.1}%", p))
                        .unwrap_or_default(),
                ),
        )
        // Files
        .child(
            div()
                .min_w(px(64.0))
                .px(px(8.0))
                .text_color(COLOR_TEXT_SECONDARY)
                .child(node.file_count.to_string()),
        )
        // Folders
        .child(
            div()
                .min_w(px(64.0))
                .px(px(8.0))
                .text_color(COLOR_TEXT_SECONDARY)
                .child(node.folder_count.to_string()),
        )
        // Modified
        .child(
            div()
                .min_w(px(120.0))
                .px(px(8.0))
                .text_color(COLOR_TEXT_SECONDARY)
                .child(node.modified.clone().unwrap_or_default()),
        )
}

/// Formats a byte count into a human-readable string.
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1_024;
    const MB: u64 = 1_024 * KB;
    const GB: u64 = 1_024 * MB;
    const TB: u64 = 1_024 * GB;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}