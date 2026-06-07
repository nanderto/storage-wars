//! Tree view — hierarchical file list with columns, chevrons, icons, and progress bars.

use gpui::{
    div, px, AppContext, Div, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
    Window,
};

use crate::{
    theme,
    types::{format_bytes, SizeChange, TreeNode},
};

/// Column definitions for the tree view header.
const COLUMNS: &[(&str, f32)] = &[
    ("Name", 280.0),
    ("% Parent", 80.0),
    ("Size", 100.0),
    ("Prev Size", 100.0),
    ("% Prev", 80.0),
    ("Files", 70.0),
    ("Folders", 70.0),
    ("Modified", 140.0),
];

/// Hierarchical file-system tree view.
pub struct TreeView {
    focus_handle: FocusHandle,
    nodes: Vec<TreeNode>,
    selected_path: Option<String>,
}

impl TreeView {
    pub fn new(nodes: Vec<TreeNode>, cx: &mut gpui::Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            nodes,
            selected_path: None,
        }
    }

    pub fn set_nodes(&mut self, nodes: Vec<TreeNode>) {
        self.nodes = nodes;
    }

    // ── rendering helpers ─────────────────────────────────────────────────────

    fn render_header(&self) -> Div {
        let cols: Vec<Div> = COLUMNS
            .iter()
            .map(|(label, width)| {
                div()
                    .flex()
                    .items_center()
                    .w(px(*width))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .text_color(theme::COLOR_TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .border_r_1()
                    .border_color(theme::COLOR_BORDER)
                    .child(*label)
            })
            .collect();

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(theme::TREE_ROW_HEIGHT))
            .bg(theme::COLOR_SURFACE)
            .border_b_1()
            .border_color(theme::COLOR_BORDER)
            .children(cols)
    }

    fn render_node(&self, node: &TreeNode) -> Div {
        let indent = node.depth as f32 * theme::TREE_INDENT_PX;
        let is_selected = self.selected_path.as_deref() == Some(&node.path);

        let chevron = if node.is_dir {
            if node.is_expanded { "▾ " } else { "▸ " }
        } else {
            "  "
        };

        let icon = if node.is_dir { "📁 " } else { "📄 " };

        let size_change = SizeChange::from_pct(node.size_change_pct());
        let bar_color = match size_change {
            SizeChange::Increased => theme::COLOR_SIZE_INCREASED,
            SizeChange::Decreased => theme::COLOR_SIZE_DECREASED,
            SizeChange::Unchanged => theme::COLOR_SIZE_UNCHANGED,
        };

        let pct_prev_text = match node.size_change_pct() {
            Some(p) => format!("{:+.1}%", p),
            None => "—".into(),
        };

        let prev_size_text = match node.prev_size_bytes {
            Some(b) => format_bytes(b),
            None => "—".into(),
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(theme::TREE_ROW_HEIGHT))
            .bg(if is_selected {
                theme::COLOR_SELECTED
            } else {
                theme::COLOR_BACKGROUND
            })
            .border_b_1()
            .border_color(theme::COLOR_BORDER)
            // Name column
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .w(px(COLUMNS[0].1))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .pl(px(theme::SPACING_SM + indent))
                    .text_color(theme::COLOR_TEXT_PRIMARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .overflow_hidden()
                    .child(format!("{}{}{}", chevron, icon, node.name)),
            )
            // % Parent column — progress bar
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .w(px(COLUMNS[1].1))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .child(
                        div()
                            .w_full()
                            .h(px(theme::PROGRESS_BAR_HEIGHT))
                            .rounded(px(2.0))
                            .bg(theme::COLOR_SURFACE_RAISED)
                            .child(
                                div()
                                    .h_full()
                                    .w(px(COLUMNS[1].1 * node.parent_fraction.clamp(0.0, 1.0)
                                        - theme::SPACING_SM * 2.0))
                                    .rounded(px(2.0))
                                    .bg(bar_color),
                            ),
                    ),
            )
            // Size column
            .child(
                div()
                    .flex()
                    .items_center()
                    .w(px(COLUMNS[2].1))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .text_color(theme::COLOR_TEXT_PRIMARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(format_bytes(node.size_bytes)),
            )
            // Prev Size column
            .child(
                div()
                    .flex()
                    .items_center()
                    .w(px(COLUMNS[3].1))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .text_color(theme::COLOR_TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(prev_size_text),
            )
            // % Prev column
            .child(
                div()
                    .flex()
                    .items_center()
                    .w(px(COLUMNS[4].1))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .text_color(match size_change {
                        SizeChange::Increased => theme::COLOR_SIZE_INCREASED,
                        SizeChange::Decreased => theme::COLOR_SIZE_DECREASED,
                        SizeChange::Unchanged => theme::COLOR_TEXT_MUTED,
                    })
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(pct_prev_text),
            )
            // Files column
            .child(
                div()
                    .flex()
                    .items_center()
                    .w(px(COLUMNS[5].1))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .text_color(theme::COLOR_TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(node.file_count.to_string()),
            )
            // Folders column
            .child(
                div()
                    .flex()
                    .items_center()
                    .w(px(COLUMNS[6].1))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .text_color(theme::COLOR_TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(node.folder_count.to_string()),
            )
            // Modified column
            .child(
                div()
                    .flex()
                    .items_center()
                    .w(px(COLUMNS[7].1))
                    .h_full()
                    .px(px(theme::SPACING_SM))
                    .text_color(theme::COLOR_TEXT_MUTED)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(node.modified.clone().unwrap_or_else(|| "—".into())),
            )
    }

    /// Flatten the tree into a depth-first ordered list of visible nodes.
    fn visible_nodes<'a>(nodes: &'a [TreeNode]) -> Vec<&'a TreeNode> {
        let mut result = Vec::new();
        for node in nodes {
            result.push(node);
            if node.is_expanded && node.is_dir {
                let children = Self::visible_nodes(&node.children);
                result.extend(children);
            }
        }
        result
    }
}

impl Render for TreeView {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let visible = Self::visible_nodes(&self.nodes);
        let rows: Vec<Div> = visible.iter().map(|n| self.render_node(n)).collect();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme::COLOR_BACKGROUND)
            .track_focus(&self.focus_handle)
            .child(self.render_header())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .children(rows),
            )
    }
}

impl Focusable for TreeView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}