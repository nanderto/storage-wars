//! [`ScanHistory`] — 280 px-wide focusable panel for managing scan records.

use gpui::*;

use crate::theme::{self, HISTORY_PANEL_WIDTH_PX};
use crate::types::{ScanRecord, SelectionRole};

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum ScanHistoryEvent {
    Compare { base_id: uuid::Uuid, new_id: uuid::Uuid },
    Delete(uuid::Uuid),
    SelectionChanged,
}

impl EventEmitter<ScanHistoryEvent> for ScanHistory {}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// A 280 px-wide panel listing historical scans with Base/New selection,
/// Compare, and Delete actions.
pub struct ScanHistory {
    records: Vec<ScanRecord>,
    base_id: Option<uuid::Uuid>,
    new_id: Option<uuid::Uuid>,
    focus_handle: FocusHandle,
}

impl ScanHistory {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            records: Vec::new(),
            base_id: None,
            new_id: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Replace the scan records list.
    pub fn set_records(&mut self, records: Vec<ScanRecord>, cx: &mut ViewContext<Self>) {
        self.records = records;
        cx.notify();
    }

    fn set_role(&mut self, id: uuid::Uuid, role: SelectionRole, cx: &mut ViewContext<Self>) {
        match role {
            SelectionRole::Base => {
                self.base_id = Some(id);
                // Prevent same record being both base and new.
                if self.new_id == Some(id) {
                    self.new_id = None;
                }
            }
            SelectionRole::New => {
                self.new_id = Some(id);
                if self.base_id == Some(id) {
                    self.base_id = None;
                }
            }
            SelectionRole::None => {}
        }
        cx.emit(ScanHistoryEvent::SelectionChanged);
        cx.notify();
    }

    fn compare(&mut self, cx: &mut ViewContext<Self>) {
        if let (Some(base), Some(new)) = (self.base_id, self.new_id) {
            cx.emit(ScanHistoryEvent::Compare { base_id: base, new_id: new });
        }
    }

    fn delete(&mut self, id: uuid::Uuid, cx: &mut ViewContext<Self>) {
        self.records.retain(|r| r.id != id);
        if self.base_id == Some(id) {
            self.base_id = None;
        }
        if self.new_id == Some(id) {
            self.new_id = None;
        }
        cx.emit(ScanHistoryEvent::Delete(id));
        cx.notify();
    }

    fn can_compare(&self) -> bool {
        self.base_id.is_some() && self.new_id.is_some()
    }

    // -----------------------------------------------------------------------
    // Rendering helpers
    // -----------------------------------------------------------------------

    fn render_record(&self, record: &ScanRecord, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let id = record.id;
        let is_base = self.base_id == Some(id);
        let is_new = self.new_id == Some(id);

        let label: SharedString = record.label.clone().into();
        let date_str: SharedString = record.scanned_at.format("%Y-%m-%d %H:%M").to_string().into();
        let size_str: SharedString = bytesize::ByteSize(record.total_bytes).to_string().into();

        div()
            .id(ElementId::Name(format!("history-row-{id}").into()))
            .flex()
            .flex_col()
            .w_full()
            .px(px(8.0))
            .py(px(6.0))
            .border_b_1()
            .border_color(theme::border())
            .bg(theme::bg_primary())
            .hover(|s| s.bg(theme::bg_hover()))
            // --- Label + size ---
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_color(theme::text_primary())
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .child(label),
                    )
                    .child(
                        div()
                            .text_color(theme::text_secondary())
                            .text_xs()
                            .child(size_str),
                    ),
            )
            // --- Date ---
            .child(
                div()
                    .text_color(theme::text_secondary())
                    .text_xs()
                    .child(date_str),
            )
            // --- Base / New / Delete buttons ---
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .mt(px(4.0))
                    // Base button
                    .child(
                        div()
                            .id(ElementId::Name(format!("base-btn-{id}").into()))
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(3.0))
                            .text_xs()
                            .cursor_pointer()
                            .bg(if is_base {
                                theme::accent()
                            } else {
                                theme::bg_secondary()
                            })
                            .text_color(if is_base {
                                theme::bg_primary()
                            } else {
                                theme::text_secondary()
                            })
                            .hover(|s| s.bg(theme::bg_hover()))
                            .on_click(cx.listener(move |this, _ev, cx| {
                                this.set_role(id, SelectionRole::Base, cx)
                            }))
                            .child("Base"),
                    )
                    // New button
                    .child(
                        div()
                            .id(ElementId::Name(format!("new-btn-{id}").into()))
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(3.0))
                            .text_xs()
                            .cursor_pointer()
                            .bg(if is_new {
                                theme::accent()
                            } else {
                                theme::bg_secondary()
                            })
                            .text_color(if is_new {
                                theme::bg_primary()
                            } else {
                                theme::text_secondary()
                            })
                            .hover(|s| s.bg(theme::bg_hover()))
                            .on_click(cx.listener(move |this, _ev, cx| {
                                this.set_role(id, SelectionRole::New, cx)
                            }))
                            .child("New"),
                    )
                    // Spacer
                    .child(div().flex_grow())
                    // Delete button
                    .child(
                        div()
                            .id(ElementId::Name(format!("del-btn-{id}").into()))
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(3.0))
                            .text_xs()
                            .cursor_pointer()
                            .bg(theme::bg_secondary())
                            .text_color(theme::destructive())
                            .hover(|s| s.bg(theme::bg_hover()))
                            .on_click(cx.listener(move |this, _ev, cx| this.delete(id, cx)))
                            .child("Delete"),
                    ),
            )
    }

    fn render_compare_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let enabled = self.can_compare();

        div()
            .id("compare-btn")
            .w_full()
            .h(px(36.0))
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .bg(if enabled {
                theme::accent()
            } else {
                theme::bg_secondary()
            })
            .text_color(if enabled {
                theme::bg_primary()
            } else {
                theme::text_secondary()
            })
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .rounded(px(4.0))
            .mx(px(8.0))
            .mb(px(8.0))
            .when(enabled, |el| {
                el.on_click(cx.listener(|this, _ev, cx| this.compare(cx)))
            })
            .child("Compare")
    }
}

// ---------------------------------------------------------------------------
// Focusable
// ---------------------------------------------------------------------------

impl Focusable for ScanHistory {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for ScanHistory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let records = self.records.clone();

        div()
            .id("scan-history")
            .flex()
            .flex_col()
            .w(px(HISTORY_PANEL_WIDTH_PX))
            .h_full()
            .bg(theme::bg_secondary())
            .border_r_1()
            .border_color(theme::border())
            .track_focus(&self.focus_handle)
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .h(px(36.0))
                    .px(px(10.0))
                    .border_b_1()
                    .border_color(theme::border())
                    .child(
                        div()
                            .text_color(theme::text_primary())
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Scan History"),
                    ),
            )
            // Record list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .overflow_y_scroll()
                    .children(records.iter().map(|r| self.render_record(r, cx))),
            )
            // Compare button pinned to bottom
            .child(self.render_compare_button(cx))
    }
}