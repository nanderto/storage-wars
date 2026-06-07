//! [`TreeView`] — hierarchical file-system tree with sortable columns.

use gpui::*;

use crate::theme::{self, TREE_INDENT_PX, TREE_ROW_HEIGHT_PX};
use crate::types::{FileNode, SizeChange};

// ---------------------------------------------------------------------------
// Column definitions
// ---------------------------------------------------------------------------

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

    pub fn width_px(self) -> f32 {
        match self {
            Column::Name => 300.0,
            Column::PercentParent => 90.0,
            Column::Size => 90.0,
            Column::PrevSize => 90.0,
            Column::PercentPrev => 80.0,
            Column::Files => 70.0,
            Column::Folders => 70.0,
            Column::Modified => 140.0,
        }
    }
}

const ALL_COLUMNS: &[Column] = &[
    Column::Name,
    Column::PercentParent,
    Column::Size,
    Column::PrevSize,
    Column::PercentPrev,
    Column::Files,
    Column::Folders,
    Column::Modified,
];

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// Events emitted by [`TreeView`].
#[derive(Debug, Clone)]
pub enum TreeViewEvent {
    NodeSelected(String),
    NodeToggled(String),
}

impl EventEmitter<TreeViewEvent> for TreeView {}

/// Hierarchical file-system tree view.
pub struct TreeView {
    roots: Vec<FileNode>,
    selected_path: Option<String>,
    focus_handle: FocusHandle,
    /// Flat list of currently visible nodes (after expansion).
    visible_nodes: Vec<VisibleNode>,
}

#[derive(Debug, Clone)]
struct VisibleNode {
    path: String,
    name: String,
    is_dir: bool,
    depth: usize,
    is_expanded: bool,
    has_children: bool,
    size_bytes: u64,
    prev_size_bytes: Option<u64>,
    parent_size_bytes: u64,
    file_count: u64,
    folder_count: u64,
    modified: Option<chrono::DateTime<chrono::Utc>>,
    size_change: SizeChange,
}

impl TreeView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            roots: Vec::new(),
            selected_path: None,
            focus_handle: cx.focus_handle(),
            visible_nodes: Vec::new(),
        }
    }

    /// Replace the root nodes and rebuild the visible list.
    pub fn set_roots(&mut self, roots: Vec<FileNode>, cx: &mut ViewContext<Self>) {
        self.roots = roots;
        self.rebuild_visible();
        cx.notify();
    }

    fn rebuild_visible(&mut self) {
        self.visible_nodes.clear();
        let roots = self.roots.clone();
        for root in &roots {
            let parent_size = root.size_bytes;
            self.collect_visible(root, parent_size);
        }
    }

    fn collect_visible(&mut self, node: &FileNode, parent_size: u64) {
        let has_children = !node.children.is_empty();

        self.visible_nodes.push(VisibleNode {
            path: node.path.clone(),
            name: node.name.clone(),
            is_dir: node.is_dir,
            depth: node.depth,
            is_expanded: node.is_expanded,
            has_children,
            size_bytes: node.size_bytes,
            prev_size_bytes: node.prev_size_bytes,
            parent_size_bytes: parent_size,
            file_count: node.file_count,
            folder_count: node.folder_count,
            modified: node.modified,
            size_change: node.size_change(),
        });

        if node.is_expanded {
            for child in &node.children {
                self.collect_visible(child, node.size_bytes);
            }
        }
    }

    fn toggle_node(&mut self, path: &str, cx: &mut ViewContext<Self>) {
        Self::toggle_in_list(&mut self.roots, path);
        self.rebuild_visible();
        cx.emit(TreeViewEvent::NodeToggled(path.to_string()));
        cx.notify();
    }

    fn toggle_in_list(nodes: &mut Vec<FileNode>, path: &str) -> bool {
        for node in nodes.iter_mut() {
            if node.path == path {
                node.is_expanded = !node.is_expanded;
                return true;
            }
            if Self::toggle_in_list(&mut node.children, path) {
                return true;
            }
        }
        false
    }

    fn select_node(&mut self, path: String, cx: &mut ViewContext<Self>) {
        self.selected_path = Some(path.clone());
        cx.emit(TreeViewEvent::NodeSelected(path));
        cx.notify();
    }

    // -----------------------------------------------------------------------
    // Rendering helpers
    // -----------------------------------------------------------------------

    fn render_header(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .h(px(TREE_ROW_HEIGHT_PX))
            .bg(theme::bg_secondary())
            .border_b_1()
            .border_color(theme::border())
            .children(ALL_COLUMNS.iter().map(|col| {
                div()
                    .w(px(col.width_px()))
                    .flex_shrink_0()
                    .h_full()
                    .flex()
                    .items_center()
                    .px(px(6.0))
                    .border_r_1()
                    .border_color(theme::border())
                    .child(
                        div()
                            .text_color(theme::text_secondary())
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(col.label()),
                    )
            }))
    }

    fn render_row(&self, node: &VisibleNode, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_selected = self.selected_path.as_deref() == Some(&node.path);
        let indent = node.depth as f32 * TREE_INDENT_PX;

        let path_for_toggle = node.path.clone();
        let path_for_select = node.path.clone();

        // --- Name cell ---
        let chevron: &'static str = if node.has_children {
            if node.is_expanded { "▾ " } else { "▸ " }
        } else {
            "  "
        };
        let icon: &'static str = if node.is_dir { "📁" } else { "📄" };

        let name_cell = div()
            .w(px(Column::Name.width_px()))
            .flex_shrink_0()
            .h_full()
            .flex()
            .items_center()
            .px(px(4.0))
            .border_r_1()
            .border_color(theme::border())
            .child(
                div()
                    .flex()
                    .items_center()
                    .pl(px(indent))
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_color(theme::text_secondary())
                            .text_xs()
                            .cursor_pointer()
                            .on_click(cx.listener({
                                let p = path_for_toggle.clone();
                                move |this, _ev, cx| this.toggle_node(&p, cx)
                            }))
                            .child(chevron),
                    )
                    .child(div().text_xs().child(icon))
                    .child(
                        div()
                            .text_color(theme::text_primary())
                            .text_sm()
                            .overflow_hidden()
                            .child(SharedString::from(node.name.clone())),
                    ),
            );

        // --- % Parent cell with progress bar ---
        let pct_parent = if node.parent_size_bytes > 0 {
            (node.size_bytes as f64 / node.parent_size_bytes as f64 * 100.0).min(100.0)
        } else {
            0.0
        };
        let bar_color = match node.size_change {
            SizeChange::Grown => theme::size_grown(),
            SizeChange::Shrunk => theme::size_shrunk(),
            SizeChange::Unchanged => theme::size_unchanged(),
        };

        let pct_parent_cell = div()
            .w(px(Column::PercentParent.width_px()))
            .flex_shrink_0()
            .h_full()
            .relative()
            .flex()
            .items_center()
            .px(px(4.0))
            .border_r_1()
            .border_color(theme::border())
            .child(
                // progress bar background
                div()
                    .absolute()
                    .left(px(0.0))
                    .top(px(0.0))
                    .h_full()
                    .w(px(Column::PercentParent.width_px() * pct_parent as f32 / 100.0))
                    .bg(bar_color)
                    .opacity(0.25),
            )
            .child(
                div()
                    .relative()
                    .text_color(theme::text_primary())
                    .text_xs()
                    .child(format!("{pct_parent:.1}%")),
            );

        // --- Size cell ---
        let size_cell = self.simple_cell(
            Column::Size,
            bytesize::ByteSize(node.size_bytes).to_string(),
        );

        // --- Prev Size cell ---
        let prev_size_cell = self.simple_cell(
            Column::PrevSize,
            node.prev_size_bytes
                .map(|b| bytesize::ByteSize(b).to_string())
                .unwrap_or_default(),
        );

        // --- % Prev cell ---
        let pct_prev_str = if let (Some(prev), cur) = (node.prev_size_bytes, node.size_bytes) {
            if prev > 0 {
                let p = (cur as f64 - prev as f64) / prev as f64 * 100.0;
                format!("{p:+.1}%")
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        let pct_prev_cell = self.simple_cell(Column::PercentPrev, pct_prev_str);

        // --- Files / Folders ---
        let files_cell = self.simple_cell(Column::Files, node.file_count.to_string());
        let folders_cell = self.simple_cell(Column::Folders, node.folder_count.to_string());

        // --- Modified ---
        let modified_str = node
            .modified
            .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_default();
        let modified_cell = self.simple_cell(Column::Modified, modified_str);

        div()
            .id(ElementId::Name(format!("row-{}", node.path).into()))
            .flex()
            .flex_row()
            .w_full()
            .h(px(TREE_ROW_HEIGHT_PX))
            .bg(if is_selected {
                theme::bg_selected()
            } else {
                theme::bg_primary()
            })
            .hover(|s| {
                if !is_selected {
                    s.bg(theme::bg_hover())
                } else {
                    s
                }
            })
            .border_b_1()
            .border_color(theme::border())
            .cursor_pointer()
            .on_click(cx.listener(move |this, _ev, cx| {
                this.select_node(path_for_select.clone(), cx)
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

    fn simple_cell(&self, col: Column, text: impl Into<SharedString>) -> Div {
        div()
            .w(px(col.width_px()))
            .flex_shrink_0()
            .h_full()
            .flex()
            .items_center()
            .px(px(6.0))
            .border_r_1()
            .border_color(theme::border())
            .child(
                div()
                    .text_color(theme::text_primary())
                    .text_xs()
                    .child(text.into()),
            )
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
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let nodes = self.visible_nodes.clone();

        div()
            .id("tree-view")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .track_focus(&self.focus_handle)
            .bg(theme::bg_primary())
            .child(self.render_header())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .flex_grow()
                    .overflow_y_scroll()
                    .children(
                        nodes
                            .iter()
                            .map(|node| self.render_row(node, cx))
                            .collect::<Vec<_>>(),
                    ),
            )
    }
}