//! Scan history panel — a 280 px-wide focusable list of past scans.

use gpui::{
    div, prelude::*, px, Context, Element, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};

use crate::theme::{
    COLOR_ACCENT, COLOR_BACKGROUND, COLOR_BORDER, COLOR_SURFACE, COLOR_TEXT_PRIMARY,
    COLOR_TEXT_SECONDARY, SCAN_HISTORY_WIDTH_PX,
};

/// Represents a single recorded scan.
#[derive(Debug, Clone)]
pub struct ScanRecord {
    pub id: u64,
    pub label: String,
    pub drive_path: String,
    pub timestamp: String,
    pub total_bytes: u64,
}

/// Selection role for a scan record in a comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanRole {
    Base,
    New,
}

/// 280 px-wide focusable panel listing scan history.
pub struct ScanHistory {
    focus_handle: FocusHandle,
    records: Vec<ScanRecord>,
    base_id: Option<u64>,
    new_id: Option<u64>,
    hovered_id: Option<u64>,
}

impl ScanHistory {
    /// Creates a new empty [`ScanHistory`].
    pub fn new() -> Self {
        panic!("ScanHistory must be constructed inside a GPUI context via ScanHistory::build(cx)")
    }

    /// Constructs the panel within a GPUI context.
    pub fn build(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            records: Vec::new(),
            base_id: None,
            new_id: None,
            hovered_id: None,
        }
    }

    /// Replaces the scan record list.
    pub fn set_records(&mut self, records: Vec<ScanRecord>) {
        self.records = records;
    }

    /// Returns the currently selected base scan, if any.
    pub fn base_record(&self) -> Option<&ScanRecord> {
        self.base_id
            .and_then(|id| self.records.iter().find(|r| r.id == id))
    }

    /// Returns the currently selected new scan, if any.
    pub fn new_record(&self) -> Option<&ScanRecord> {
        self.new_id
            .and_then(|id| self.records.iter().find(|r| r.id == id))
    }

    /// Returns `true` if both a base and a new scan are selected.
    pub fn can_compare(&self) -> bool {
        self.base_id.is_some() && self.new_id.is_some() && self.base_id != self.new_id
    }
}

impl Default for ScanHistory {
    fn default() -> Self {
        panic!("ScanHistory must be constructed with ScanHistory::build(cx)")
    }
}

impl Focusable for ScanHistory {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ScanHistory {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(SCAN_HISTORY_WIDTH_PX))
            .h_full()
            .bg(COLOR_SURFACE)
            .border_r_1()
            .border_color(COLOR_BORDER)
            // Panel header
            .child(self.render_panel_header())
            // Scrollable scan list
            .child(self.render_scan_list())
            // Action buttons
            .child(self.render_action_buttons())
    }
}

impl ScanHistory {
    fn render_panel_header(&self) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .w_full()
            .h(px(32.0))
            .px(px(8.0))
            .border_b_1()
            .border_color(COLOR_BORDER)
            .text_color(COLOR_TEXT_SECONDARY)
            .child("Scan History")
    }

    fn render_scan_list(&self) -> impl IntoElement {
        let mut list = div()
            .flex()
            .flex_col()
            .flex_1()
            .overflow_hidden()
            .bg(COLOR_BACKGROUND);

        if self.records.is_empty() {
            list = list.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .flex_1()
                    .text_color(COLOR_TEXT_SECONDARY)
                    .child("No scans yet"),
            );
        } else {
            for record in &self.records {
                list = list.child(self.render_scan_row(record));
            }
        }

        list
    }

    fn render_scan_row(&self, record: &ScanRecord) -> impl IntoElement {
        let is_base = self.base_id == Some(record.id);
        let is_new = self.new_id == Some(record.id);

        let role_badge = if is_base {
            Some("Base")
        } else if is_new {
            Some("New")
        } else {
            None
        };

        let badge_color = if is_base {
            COLOR_ACCENT
        } else {
            COLOR_TEXT_SECONDARY
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .px(px(8.0))
            .py(px(4.0))
            .border_b_1()
            .border_color(COLOR_BORDER)
            // Row header: label + optional role badge
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_color(COLOR_TEXT_PRIMARY)
                            .child(record.label.clone()),
                    )
                    .child(
                        div()
                            .text_color(badge_color)
                            .child(role_badge.unwrap_or("")),
                    ),
            )
            // Subtitle: drive + timestamp
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .text_color(COLOR_TEXT_SECONDARY)
                    .child(record.drive_path.clone())
                    .child(record.timestamp.clone()),
            )
    }

    fn render_action_buttons(&self) -> impl IntoElement {
        let can_compare = self.can_compare();

        div()
            .flex()
            .flex_row()
            .gap(px(8.0))
            .w_full()
            .h(px(40.0))
            .px(px(8.0))
            .items_center()
            .border_t_1()
            .border_color(COLOR_BORDER)
            .child(self.render_button("Compare", can_compare))
            .child(self.render_button("Delete", !self.records.is_empty()))
    }

    fn render_button(&self, label: &'static str, enabled: bool) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .px(px(10.0))
            .py(px(3.0))
            .rounded(px(4.0))
            .border_1()
            .border_color(if enabled { COLOR_ACCENT } else { COLOR_BORDER })
            .text_color(if enabled {
                COLOR_ACCENT
            } else {
                COLOR_TEXT_SECONDARY
            })
            .child(label)
    }
}