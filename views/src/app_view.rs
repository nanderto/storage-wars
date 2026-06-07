//! Root application view that orchestrates all sub-components.

use gpui::{
    div, px, Context, Entity, IntoElement, ParentElement, Render, Styled, ViewContext,
    VisualContext,
};

use crate::{
    drive_selector::DriveSelector,
    scan_history::ScanHistory,
    theme::{Palette, SCAN_HISTORY_WIDTH_PX, TITLE_BAR_HEIGHT_PX},
    tree_view::TreeView,
};

/// Root orchestrator view.
///
/// Hosts:
/// - Custom title bar with window controls
/// - [`DriveSelector`] for choosing the drive to scan
/// - [`TreeView`] for the hierarchical file listing
/// - [`ScanHistory`] panel (280 px wide) on the right
/// - Drive info panel below the selector
pub struct AppView {
    drive_selector: Entity<DriveSelector>,
    tree_view: Entity<TreeView>,
    scan_history: Entity<ScanHistory>,
}

impl AppView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            drive_selector: cx.new(|cx| DriveSelector::new(cx)),
            tree_view: cx.new(|cx| TreeView::new(cx)),
            scan_history: cx.new(|cx| ScanHistory::new(cx)),
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(Palette::background())
            // ── Title bar ──────────────────────────────────────────────────
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .h(px(TITLE_BAR_HEIGHT_PX))
                    .bg(Palette::title_bar())
                    .border_b_1()
                    .border_color(Palette::border())
                    .px(px(12.0))
                    .child(
                        div()
                            .text_color(Palette::text_primary())
                            .text_sm()
                            .child("Disk Space Analyzer"),
                    ),
            )
            // ── Toolbar row: drive selector ─────────────────────────────
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .w_full()
                    .h(px(48.0))
                    .bg(Palette::surface())
                    .border_b_1()
                    .border_color(Palette::border())
                    .px(px(12.0))
                    .gap(px(8.0))
                    .child(self.drive_selector.clone()),
            )
            // ── Main content area ───────────────────────────────────────
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .overflow_hidden()
                    // Tree view takes remaining width
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child(self.tree_view.clone()),
                    )
                    // Scan history panel – fixed 280 px
                    .child(
                        div()
                            .w(px(SCAN_HISTORY_WIDTH_PX))
                            .h_full()
                            .border_l_1()
                            .border_color(Palette::border())
                            .child(self.scan_history.clone()),
                    ),
            )
    }
}