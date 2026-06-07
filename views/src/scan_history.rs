//! Scan history panel — 280px-wide focusable list with Base/New selection,
//! Compare and Delete action buttons.

use gpui::{
    div, px, AppContext, Div, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
    Window,
};

use crate::{
    theme,
    types::{format_bytes, HistorySelection, ScanRecord},
};

/// Scan history side panel.
pub struct ScanHistory {
    focus_handle: FocusHandle,
    records: Vec<ScanRecord>,
    selection: HistorySelection,
}

impl ScanHistory {
    pub fn new(records: Vec<ScanRecord>, cx: &mut gpui::Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            records,
            selection: HistorySelection::default(),
        }
    }

    pub fn set_records(&mut self, records: Vec<ScanRecord>) {
        self.records = records;
    }

    // ── rendering helpers ─────────────────────────────────────────────────────

    fn render_record_row(&self, record: &ScanRecord) -> Div {
        let is_base = self.selection.base_id == Some(record.id);
        let is_new = self.selection.new_id == Some(record.id);

        let badge = if is_base {
            Some("Base")
        } else if is_new {
            Some("New")
        } else {
            None
        };

        let badge_color = if is_base {
            theme::COLOR_ACCENT
        } else {
            theme::COLOR_SIZE_DECREASED
        };

        let row_bg = if is_base || is_new {
            theme::COLOR_SELECTED
        } else {
            theme::COLOR_SURFACE
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .h(px(theme::HISTORY_ROW_HEIGHT))
            .px(px(theme::SPACING_MD))
            .py(px(theme::SPACING_XS))
            .bg(row_bg)
            .border_b_1()
            .border_color(theme::COLOR_BORDER)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .child(
                        div()
                            .text_color(theme::COLOR_TEXT_PRIMARY)
                            .text_size(px(theme::FONT_SIZE_SM))
                            .overflow_hidden()
                            .child(record.label.clone()),
                    )
                    .child(if let Some(b) = badge {
                        div()
                            .px(px(theme::SPACING_XS))
                            .py(px(2.0))
                            .rounded(px(3.0))
                            .bg(badge_color)
                            .text_color(theme::COLOR_BACKGROUND)
                            .text_size(px(10.0))
                            .child(b)
                    } else {
                        div()
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .child(
                        div()
                            .text_color(theme::COLOR_TEXT_MUTED)
                            .text_size(px(10.0))
                            .child(record.scanned_at.clone()),
                    )
                    .child(
                        div()
                            .text_color(theme::COLOR_TEXT_SECONDARY)
                            .text_size(px(10.0))
                            .child(format_bytes(record.total_bytes)),
                    ),
            )
    }

    fn render_action_buttons(&self) -> Div {
        let can_compare =
            self.selection.base_id.is_some() && self.selection.new_id.is_some();

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(theme::SPACING_SM))
            .w_full()
            .p(px(theme::SPACING_SM))
            .bg(theme::COLOR_SURFACE)
            .border_t_1()
            .border_color(theme::COLOR_BORDER)
            .child(
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .h(px(28.0))
                    .rounded(px(4.0))
                    .bg(if can_compare {
                        theme::COLOR_ACCENT
                    } else {
                        theme::COLOR_BUTTON_BG
                    })
                    .text_color(if can_compare {
                        theme::COLOR_BACKGROUND
                    } else {
                        theme::COLOR_TEXT_MUTED
                    })
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child("Compare"),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(28.0))
                    .px(px(theme::SPACING_MD))
                    .rounded(px(4.0))
                    .bg(theme::COLOR_BUTTON_BG)
                    .text_color(theme::COLOR_BUTTON_DANGER)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child("Delete"),
            )
    }
}

impl Render for ScanHistory {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let rows: Vec<Div> = self
            .records
            .iter()
            .map(|r| self.render_record_row(r))
            .collect();

        div()
            .flex()
            .flex_col()
            .w(px(theme::SCAN_HISTORY_WIDTH))
            .h_full()
            .bg(theme::COLOR_SURFACE)
            .border_r_1()
            .border_color(theme::COLOR_BORDER)
            .track_focus(&self.focus_handle)
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .w_full()
                    .h(px(theme::TREE_ROW_HEIGHT))
                    .px(px(theme::SPACING_MD))
                    .bg(theme::COLOR_SURFACE)
                    .border_b_1()
                    .border_color(theme::COLOR_BORDER)
                    .text_color(theme::COLOR_TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child("Scan History"),
            )
            // Scrollable list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .w_full()
                    .overflow_y_scroll()
                    .children(rows),
            )
            // Action buttons pinned to bottom
            .child(self.render_action_buttons())
    }
}

impl Focusable for ScanHistory {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}