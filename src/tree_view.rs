use gpui::prelude::*;
use gpui::{
    div, px, relative, rgb, App, ClickEvent, Context, ElementId, EventEmitter, Focusable,
    FocusHandle, IntoElement, Render, Rgba, SharedString, Window,
};

use crate::models::{format_size, SizeChange, UiNode};

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

pub enum TreeViewEvent {
    ToggleExpand(std::path::PathBuf),
}

impl EventEmitter<TreeViewEvent> for TreeView {}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

pub struct TreeView {
    pub nodes: Vec<UiNode>,
    focus_handle: FocusHandle,
}

impl TreeView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            nodes: Vec::new(),
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_nodes(&mut self, nodes: Vec<UiNode>, cx: &mut Context<Self>) {
        self.nodes = nodes;
        cx.notify();
    }
}

impl Focusable for TreeView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn bar_color(change: SizeChange) -> Rgba {
    match change {
        SizeChange::Decreased => rgb(0x22c55e),
        SizeChange::Unchanged | SizeChange::NoBaseline => rgb(0x6b7280),
        SizeChange::SmallGrowth => rgb(0xeab308),
        SizeChange::MediumGrowth => rgb(0xf97316),
        SizeChange::LargeGrowth => rgb(0xef4444),
    }
}

// Plain data snapshot for a single row.
struct RowSnapshot {
    ix: usize,
    indent: f32,
    path: std::path::PathBuf,
    is_dir: bool,
    chevron: SharedString,
    icon: SharedString,
    name: SharedString,
    size_label: SharedString,
    scan_progress: f32,
    bar_color: Rgba,
}

impl Render for TreeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshots: Vec<RowSnapshot> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(ix, node)| {
                let change = SizeChange::from_node(&node.fs_node);
                RowSnapshot {
                    ix,
                    indent: node.depth as f32 * 16.0,
                    path: node.fs_node.path.clone(),
                    is_dir: node.fs_node.is_dir,
                    chevron: if !node.fs_node.is_dir {
                        "  ".into()
                    } else if node.expanded {
                        "▼ ".into()
                    } else {
                        "▶ ".into()
                    },
                    icon: if node.fs_node.is_dir {
                        "📁 ".into()
                    } else {
                        "📄 ".into()
                    },
                    name: node.fs_node.name.clone().into(),
                    size_label: format_size(node.fs_node.current_size).into(),
                    scan_progress: node.scan_progress,
                    bar_color: bar_color(change),
                }
            })
            .collect();

        let rows = snapshots.into_iter().map(|row| {
            let path = row.path.clone();
            let is_dir = row.is_dir;

            div()
                .id(ElementId::Integer(row.ix as u64))
                .flex()
                .items_center()
                .w_full()
                .h(px(28.))
                .hover(|s| s.bg(rgb(0x313244)))
                .cursor_pointer()
                .on_click(cx.listener(move |_, _: &ClickEvent, _, cx| {
                    if is_dir {
                        cx.emit(TreeViewEvent::ToggleExpand(path.clone()));
                    }
                }))
                // Indent + chevron + icon + name
                .child(
                    div()
                        .flex()
                        .items_center()
                        .flex_shrink_0()
                        .pl(px(row.indent))
                        .child(
                            div()
                                .text_color(rgb(0x6c7086))
                                .text_xs()
                                .child(row.chevron),
                        )
                        .child(div().text_xs().child(row.icon))
                        .child(
                            div()
                                .text_color(rgb(0xcdd6f4))
                                .text_sm()
                                .child(row.name),
                        ),
                )
                // Spacer
                .child(div().flex_grow())
                // Size label
                .child(
                    div()
                        .flex_shrink_0()
                        .px_2()
                        .text_color(rgb(0xa6adc8))
                        .text_xs()
                        .child(row.size_label),
                )
                // Progress bar
                .child(
                    div()
                        .flex_shrink_0()
                        .w(px(120.))
                        .h(px(8.))
                        .mr(px(8.))
                        .rounded_sm()
                        .bg(rgb(0x313244))
                        .child(
                            div()
                                .h_full()
                                .rounded_sm()
                                .w(relative(row.scan_progress))
                                .bg(row.bar_color),
                        ),
                )
        });

        div()
            .id("tree-view")
            .flex()
            .flex_col()
            .flex_grow()
            .h_full()
            .bg(rgb(0x1e1e2e))
            .overflow_y_scroll()
            .when(self.nodes.is_empty(), |el: gpui::Stateful<gpui::Div>| {
                el.child(
                    div()
                        .px_4()
                        .py_6()
                        .text_color(rgb(0x6c7086))
                        .child("Select a drive and click Scan Now."),
                )
            })
            .children(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{FsNode, UiNode};
    use std::{cell::RefCell, path::PathBuf, rc::Rc};

    fn make_dir_node(name: &str, path: &str, size: u64) -> UiNode {
        UiNode {
            fs_node: FsNode {
                name: name.into(),
                path: PathBuf::from(path),
                is_dir: true,
                current_size: size,
                prev_size: None,
                children: vec![],
                file_count: 0,
                folder_count: 0,
                modified: None,
            },
            depth: 0,
            expanded: false,
            scan_progress: 1.0,
        }
    }

    fn make_file_node(name: &str, path: &str, size: u64) -> UiNode {
        UiNode {
            fs_node: FsNode {
                name: name.into(),
                path: PathBuf::from(path),
                is_dir: false,
                current_size: size,
                prev_size: None,
                children: vec![],
                file_count: 0,
                folder_count: 0,
                modified: None,
            },
            depth: 0,
            expanded: false,
            scan_progress: 0.5,
        }
    }

    #[gpui::test]
    fn initial_state_is_empty(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| TreeView::new(cx));
        view.read_with(cx, |v, _| {
            assert!(v.nodes.is_empty());
        });
    }

    #[gpui::test]
    fn set_nodes_updates_list(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| TreeView::new(cx));
        let nodes = vec![
            make_dir_node("docs", "C:/docs", 1000),
            make_file_node("readme.txt", "C:/readme.txt", 500),
        ];
        view.update(cx, |v, cx| v.set_nodes(nodes, cx));
        cx.run_until_parked();
        view.read_with(cx, |v, _| {
            assert_eq!(v.nodes.len(), 2);
            assert_eq!(v.nodes[0].fs_node.name, "docs");
            assert_eq!(v.nodes[1].fs_node.name, "readme.txt");
        });
    }

    #[gpui::test]
    fn toggle_expand_emits_event(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| TreeView::new(cx));
        view.update(cx, |v, cx| {
            v.set_nodes(vec![make_dir_node("docs", "C:/docs", 1000)], cx);
        });

        let received: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(vec![]));
        let captured = received.clone();
        let _sub = cx.update(|_window, app| {
            app.subscribe(&view, move |_, event: &TreeViewEvent, _| {
                let TreeViewEvent::ToggleExpand(path) = event;
                captured.borrow_mut().push(path.clone());
            })
        });

        view.update(cx, |_, cx| {
            cx.emit(TreeViewEvent::ToggleExpand(PathBuf::from("C:/docs")));
        });
        cx.run_until_parked();

        assert_eq!(*received.borrow(), vec![PathBuf::from("C:/docs")]);
    }

    #[gpui::test]
    fn render_does_not_panic(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| TreeView::new(cx));
        let nodes = vec![
            make_dir_node("docs", "C:/docs", 2000),
            make_file_node("readme.txt", "C:/readme.txt", 500),
        ];
        view.update(cx, |v, cx| v.set_nodes(nodes, cx));
        cx.run_until_parked();
    }
}
