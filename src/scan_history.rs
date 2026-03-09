use gpui::prelude::*;
use gpui::{
    div, px, rgb, App, ClickEvent, Context, ElementId, EventEmitter, Focusable, FocusHandle,
    IntoElement, Render, SharedString, Window,
};

use crate::models::{format_size, ScanMeta};

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

pub enum ScanHistoryEvent {
    CompareRequested { base_id: i64, new_id: i64 },
    DeleteRequested(i64),
}

impl EventEmitter<ScanHistoryEvent> for ScanHistory {}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

pub struct ScanHistory {
    pub scans: Vec<ScanMeta>,
    pub compare_a: Option<i64>,
    pub compare_b: Option<i64>,
    focus_handle: FocusHandle,
}

impl ScanHistory {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            scans: Vec::new(),
            compare_a: None,
            compare_b: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_scans(&mut self, scans: Vec<ScanMeta>, cx: &mut Context<Self>) {
        self.scans = scans;
        cx.notify();
    }
}

impl Focusable for ScanHistory {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ScanHistory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let can_compare = self.compare_a.is_some() && self.compare_b.is_some();
        let compare_a = self.compare_a;
        let compare_b = self.compare_b;

        // Snapshot state for each row so closures don't re-borrow self.
        struct RowData {
            id: i64,
            label: SharedString,
            is_a: bool,
            is_b: bool,
        }

        let row_data: Vec<RowData> = self
            .scans
            .iter()
            .map(|s| RowData {
                id: s.id,
                label: format!("{} ({})", s.scanned_at, format_size(s.total_size)).into(),
                is_a: self.compare_a == Some(s.id),
                is_b: self.compare_b == Some(s.id),
            })
            .collect();

        let rows = row_data.into_iter().map(|row| {
            let id = row.id;
            let base_bg = if row.is_a { rgb(0x89b4fa) } else { rgb(0x313244) };
            let new_bg = if row.is_b { rgb(0xa6e3a1) } else { rgb(0x313244) };

            div()
                .flex()
                .flex_col()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(rgb(0x313244))
                .child(div().text_color(rgb(0xcdd6f4)).text_sm().child(row.label))
                .child(
                    div()
                        .flex()
                        .gap_2()
                        .mt_1()
                        .child(
                            div()
                                .id(ElementId::NamedInteger("base".into(), id as u64))
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .bg(base_bg)
                                .text_color(rgb(0x1e1e2e))
                                .text_xs()
                                .child("Base")
                                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                                    this.compare_a = Some(id);
                                    cx.notify();
                                })),
                        )
                        .child(
                            div()
                                .id(ElementId::NamedInteger("new".into(), id as u64))
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .bg(new_bg)
                                .text_color(rgb(0x1e1e2e))
                                .text_xs()
                                .child("New")
                                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                                    this.compare_b = Some(id);
                                    cx.notify();
                                })),
                        )
                        .child(
                            div()
                                .id(ElementId::NamedInteger("del".into(), id as u64))
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .cursor_pointer()
                                .bg(rgb(0x45475a))
                                .text_color(rgb(0xf38ba8))
                                .text_xs()
                                .child("Delete")
                                .on_click(cx.listener(move |_, _: &ClickEvent, _, cx| {
                                    cx.emit(ScanHistoryEvent::DeleteRequested(id));
                                })),
                        ),
                )
        });

        div()
            .flex()
            .flex_col()
            .w(px(280.))
            .h_full()
            .bg(rgb(0x181825))
            .border_r_1()
            .border_color(rgb(0x313244))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(rgb(0x313244))
                    .child(
                        div()
                            .text_color(rgb(0xa6adc8))
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("SCAN HISTORY"),
                    )
                    .child(
                        div()
                            .id("compare-btn")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .when(can_compare, |el| el.bg(rgb(0x89b4fa)))
                            .when(!can_compare, |el| el.bg(rgb(0x313244)))
                            .text_color(rgb(0x1e1e2e))
                            .text_xs()
                            .child("Compare")
                            .on_click(cx.listener(move |_, _: &ClickEvent, _, cx| {
                                if let (Some(a), Some(b)) = (compare_a, compare_b) {
                                    cx.emit(ScanHistoryEvent::CompareRequested {
                                        base_id: a,
                                        new_id: b,
                                    });
                                }
                            })),
                    ),
            )
            .when(self.scans.is_empty(), |el| {
                el.child(
                    div()
                        .px_3()
                        .py_4()
                        .text_color(rgb(0x6c7086))
                        .text_sm()
                        .child("No scans yet."),
                )
            })
            .children(rows)
    }
}
