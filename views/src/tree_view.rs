//! [`TreeView`] — hierarchical file-system tree with column headers,
//! chevrons, icons, 16 px/depth indentation, and SizeChange progress bars.

use gpui::{
    div, px, AnyElement, AppContext, Context, Element, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled,
    Window,
};

use crate::theme;
use crate::types::{FileNode, SizeChange, TreeColumn};

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

pub enum TreeViewEvent {
    NodeSelected(Vec<String>),
    NodeToggled(Vec<String>),
}

impl EventEmitter<TreeViewEvent> for TreeView {}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

pub struct TreeView {
    roots: Vec<FileNode>,
    selected_path: Option<Vec<String>>,
    focus_handle: FocusHandle,
}

impl TreeView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            roots: Vec::new(),
            selected_path: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_roots(&mut self, roots: Vec<FileNode>) {
        self.roots = roots;
    }

    // -----------------------------------------------------------------------
    // Rendering helpers
    // -----------------------------------------------------------------------

    fn render_header(&self) -> impl IntoElement {
        let headers: Vec<AnyElement> = TreeColumn::ALL
            .iter()
            .map(|col| {
                div()
                    .w(px(col.default_width()))
                    .px(px(theme::SPACING_XS))
                    .flex_shrink_0()
                    .text_color(theme::TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(col.header())
                    .into_any_element()
            })
            .collect();

        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(theme::TREE_HEADER_HEIGHT))
            .bg(theme::SURFACE)
            .border_b_1()
            .border_color(theme::BORDER)
            .children(headers)
    }

    fn render_node(
        &self,
        node: &FileNode,
        depth: usize,
        path: Vec<String>,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let mut elements: Vec<AnyElement> = Vec::new();

        let indent = depth as f32 * theme::TREE_INDENT_PX;
        let is_selected = self.selected_path.as_deref() == Some(&path);
        let node_path = path.clone();
        let toggle_path = path.clone();

        let bg = if is_selected {
            theme::TREE_ROW_SELECTED
        } else {
            theme::BACKGROUND
        };

        // Chevron / icon
        let chevron: &'static str = if node.is_dir {
            if node.expanded { "▼" } else { "▶" }
        } else {
            " "
        };

        let icon: &'static str = if node.is_dir { "📁" } else { "📄" };

        // Name cell with indentation + chevron + icon
        let name_cell = div()
            .w(px(TreeColumn::Name.default_width()))
            .flex_shrink_0()
            .flex()
            .flex_row()
            .items_center()
            .pl(px(indent + theme::SPACING_XS))
            .gap(px(4.0))
            .child(
                div()
                    .text_color(theme::TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .w(px(12.0))
                    .child(chevron),
            )
            .child(
                div()
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(icon),
            )
            .child(
                div()
                    .text_color(theme::TEXT_PRIMARY)
                    .text_size(px(theme::FONT_SIZE_MD))
                    .overflow_hidden()
                    .child(SharedString::from(node.name.clone())),
            );

        // % Parent cell with progress bar
        let pct_parent = (node.parent_fraction * 100.0) as u32;
        let bar_color = theme::size_change_color(node.size_change);

        let pct_parent_cell = div()
            .w(px(TreeColumn::PercentParent.default_width()))
            .flex_shrink_0()
            .px(px(theme::SPACING_XS))
            .flex()
            .flex_col()
            .justify_center()
            .gap(px(2.0))
            .child(
                div()
                    .text_color(theme::TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(format!("{pct_parent}%")),
            )
            .child(
                // Progress bar track
                div()
                    .w_full()
                    .h(px(3.0))
                    .bg(theme::BORDER)
                    .rounded(px(2.0))
                    .child(
                        div()
                            .w(gpui::relative(node.parent_fraction))
                            .h_full()
                            .bg(bar_color)
                            .rounded(px(2.0)),
                    ),
            );

        // Size cell
        let size_label: SharedString =
            bytesize::ByteSize(node.size_bytes).to_string_as(true).into();
        let size_cell = self.simple_cell(TreeColumn::Size, size_label);

        // Prev size cell
        let prev_size_label: SharedString = node
            .prev_size_bytes
            .map(|b| bytesize::ByteSize(b).to_string_as(true))
            .unwrap_or_default()
            .into();
        let prev_size_cell = self.simple_cell(TreeColumn::PrevSize, prev_size_label);

        // % Prev cell
        let pct_prev_label: SharedString = node
            .percent_change()
            .map(|p| format!("{:+.1}%", p))
            .unwrap_or_default()
            .into();
        let pct_prev_cell = self.simple_cell(TreeColumn::PercentPrev, pct_prev_label);

        // Files cell
        let files_cell =
            self.simple_cell(TreeColumn::Files, SharedString::from(node.file_count.to_string()));

        // Folders cell
        let folders_cell = self.simple_cell(
            TreeColumn::Folders,
            SharedString::from(node.folder_count.to_string()),
        );

        // Modified cell
        let modified_label: SharedString = node
            .modified_at
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_default()
            .into();
        let modified_cell = self.simple_cell(TreeColumn::Modified, modified_label);

        let row = div()
            .id(("tree-row", node.id.as_u128() as usize))
            .flex()
            .flex_row()
            .items_center()
            .h(px(theme::TREE_ROW_HEIGHT))
            .bg(bg)
            .hover(|s| s.bg(theme::TREE_ROW_HOVER))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _ev, _window, cx| {
                this.selected_path = Some(node_path.clone());
                cx.emit(TreeViewEvent::NodeSelected(node_path.clone()));
                cx.notify();
            }))
            .child(name_cell)
            .child(pct_parent_cell)
            .child(size_cell)
            .child(prev_size_cell)
            .child(pct_prev_cell)
            .child(files_cell)
            .child(folders_cell)
            .child(modified_cell)
            .into_any_element();

        elements.push(row);

        // Recurse into children if expanded
        if node.is_dir && node.expanded {
            for child in &node.children {
                let mut child_path = path.clone();
                child_path.push(child.name.clone());
                let child_elements = self.render_node(child, depth + 1, child_path, cx);
                elements.extend(child_elements);
            }
        }

        elements
    }

    fn simple_cell(&self, col: TreeColumn, label: SharedString) -> AnyElement {
        div()
            .w(px(col.default_width()))
            .flex_shrink_0()
            .px(px(theme::SPACING_XS))
            .text_color(theme::TEXT_SECONDARY)
            .text_size(px(theme::FONT_SIZE_MD))
            .overflow_hidden()
            .child(label)
            .into_any_element()
    }
}

// ---------------------------------------------------------------------------
// Focusable
// ---------------------------------------------------------------------------

impl Focusable for TreeView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for TreeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = self.render_header();

        // Collect all visible rows
        let rows: Vec<AnyElement> = self
            .roots
            .iter()
            .flat_map(|root| {
                let path = vec![root.name.clone()];
                self.render_node(root, 0, path, cx)
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme::BACKGROUND)
            .track_focus(&self.focus_handle)
            .child(header)
            .child(
                div()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(rows),
            )
    }
}