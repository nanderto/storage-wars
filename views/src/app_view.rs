//! Root application view that orchestrates all child components.

use gpui::{
    div, prelude::*, px, Context, Element, Entity, IntoElement, ParentElement, Render, Styled,
    Window,
};

use crate::{
    drive_selector::DriveSelector, scan_history::ScanHistory, theme::COLOR_BACKGROUND,
    tree_view::TreeView,
};

/// Root orchestrator view.
///
/// Lays out:
/// - Custom title bar with window controls
/// - Drive selector toolbar
/// - Main content area: [`ScanHistory`] panel + [`TreeView`]
/// - Drive info panel / status bar
pub struct AppView {
    drive_selector: Entity<DriveSelector>,
    tree_view: Entity<TreeView>,
    scan_history: Entity<ScanHistory>,
}

impl AppView {
    /// Constructs the view and all child entities within the given context.
    pub fn build(cx: &mut Context<Self>) -> Self {
        let drive_selector = cx.new(|cx| DriveSelector::build(cx));
        let tree_view = cx.new(|cx| TreeView::build(cx));
        let scan_history = cx.new(|cx| ScanHistory::build(cx));

        Self {
            drive_selector,
            tree_view,
            scan_history,
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(COLOR_BACKGROUND)
            // Title bar
            .child(self.render_title_bar(cx))
            // Toolbar row
            .child(self.render_toolbar(cx))
            // Main content
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .child(self.scan_history.clone())
                    .child(self.tree_view.clone()),
            )
            // Status / drive info bar
            .child(self.render_status_bar(cx))
    }
}

impl AppView {
    fn render_title_bar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        use crate::theme::{COLOR_BORDER, COLOR_SURFACE, COLOR_TEXT_PRIMARY};

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(32.0))
            .bg(COLOR_SURFACE)
            .border_b_1()
            .border_color(COLOR_BORDER)
            .child(
                div()
                    .pl(px(12.0))
                    .text_color(COLOR_TEXT_PRIMARY)
                    .child("Disk Space Analyzer"),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .pr(px(8.0))
                    .child(self.render_window_button("–"))
                    .child(self.render_window_button("□"))
                    .child(self.render_window_button("✕")),
            )
    }

    fn render_window_button(&self, label: &'static str) -> impl IntoElement {
        use crate::theme::{COLOR_BORDER, COLOR_TEXT_SECONDARY};

        div()
            .w(px(28.0))
            .h(px(22.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(4.0))
            .border_1()
            .border_color(COLOR_BORDER)
            .text_color(COLOR_TEXT_SECONDARY)
            .child(label)
    }

    fn render_toolbar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        use crate::theme::{COLOR_BORDER, COLOR_SURFACE};

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .w_full()
            .h(px(40.0))
            .px(px(8.0))
            .bg(COLOR_SURFACE)
            .border_b_1()
            .border_color(COLOR_BORDER)
            .child(self.drive_selector.clone())
            .child(self.render_scan_button())
    }

    fn render_scan_button(&self) -> impl IntoElement {
        use crate::theme::{COLOR_ACCENT, COLOR_BACKGROUND};

        div()
            .px(px(12.0))
            .py(px(4.0))
            .rounded(px(4.0))
            .bg(COLOR_ACCENT)
            .text_color(COLOR_BACKGROUND)
            .child("Scan")
    }

    fn render_status_bar(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        use crate::theme::{COLOR_BORDER, COLOR_SURFACE, COLOR_TEXT_SECONDARY};

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(24.0))
            .px(px(8.0))
            .bg(COLOR_SURFACE)
            .border_t_1()
            .border_color(COLOR_BORDER)
            .text_color(COLOR_TEXT_SECONDARY)
            .child("Ready")
    }
}