//! Scan history panel – 280px wide, focusable, with Base/New selection,
//! Compare and Delete buttons.

use gpui::{
    div, px, Context, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
    ViewContext,
};

use crate::theme::{Palette, SCAN_HISTORY_WIDTH_PX};

/// Represents a single scan entry in the history list.
#[derive(Debug, Clone)]
pub struct ScanEntry {
    pub id: u64,
    pub label: String,
    pub timestamp: String,
    pub total_bytes: u64,
}

/// Role assigned to a scan entry for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanRole {
    Base,
    New,
}

/// 280px-wide focusable scan history panel.
pub struct ScanHistory {
    focus_handle: FocusHandle,
    scans: Vec<ScanEntry>,
    base_id: Option<u64>,
    new_id: Option<u64>,
}

impl ScanHistory {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            scans: Vec::new(),
            base_id: None,
            new_id: None,
        }
    }

    /// Replaces the scan list.
    pub fn set_scans(&mut self, scans: Vec<ScanEntry>, cx: &mut ViewContext<Self>) {
        self.scans = scans;
        cx.notify();
    }

    /// Assigns a role to a scan entry.
    pub fn assign_role(&mut self, id: u64, role: ScanRole, cx: &mut ViewContext<Self>) {
        match role {
            ScanRole::Base => self.base_id = Some(id),
            ScanRole::New => self.new_id = Some(id),
        }
        cx.notify();
    }

    fn on_compare(&mut self, cx: &mut ViewContext<Self>) {
        log::info!(
            "Compare requested: base={:?} new={:?}",
            self.base_id,
            self.new_id
        );
        cx.notify();
    }

    fn on_delete(&mut self, cx: &mut ViewContext<Self>) {
        // Delete whichever scan is selected as "new", if any.
        if let Some(new_id) = self.new_id {
            self.scans.retain(|s| s.id != new_id);
            self.new_id = None;
            log::info!("Deleted scan id={}", new_id);
        }
        cx.notify();
    }

    fn assign_base(&mut self, id: u64, cx: &mut ViewContext<Self>) {
        self.assign_role(id, ScanRole::Base, cx);
    }

    fn assign_new(&mut self, id: u64, cx: &mut ViewContext<Self>) {
        self.assign_role(id, ScanRole::New, cx);
    }
}

impl Focusable for ScanHistory {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ScanHistory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let rows: Vec<_> = self
            .scans
            .iter()
            .map(|scan| {
                let scan_id = scan.id;
                let is_base = self.base_id == Some(scan_id);
                let is_new = self.new_id == Some(scan_id);

                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .px(px(8.0))
                    .py(px(6.0))
                    .border_b_1()
                    .border_color(Palette::border())
                    .bg(if is_base || is_new {
                        Palette::selection()
                    } else {
                        Palette::background()
                    })
                    .hover(|s| s.bg(Palette::surface()))
                    // ── Scan label & timestamp ──────────────────────────
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .child(
                                div()
                                    .text_color(Palette::text_primary())
                                    .text_sm()
                                    .child(scan.label.clone()),
                            )
                            .child(
                                div()
                                    .text_color(Palette::text_muted())
                                    .text_xs()
                                    .child(scan.timestamp.clone()),
                            ),
                    )
                    // ── Base / New role buttons ─────────────────────────
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(4.0))
                            .mt(px(4.0))
                            .child(
                                div()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(3.0))
                                    .bg(if is_base {
                                        Palette::accent()
                                    } else {
                                        Palette::surface_elevated()
                                    })
                                    .border_1()
                                    .border_color(Palette::border())
                                    .cursor_pointer()
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(move |this, _, cx| {
                                            this.assign_base(scan_id, cx);
                                        }),
                                    )
                                    .child(
                                        div()
                                            .text_color(if is_base {
                                                Palette::background()
                                            } else {
                                                Palette::text_secondary()
                                            })
                                            .text_xs()
                                            .child("Base"),
                                    ),
                            )
                            .child(
                                div()
                                    .px(px(6.0))
                                    .py(px(2.0))
                                    .rounded(px(3.0))
                                    .bg(if is_new {
                                        Palette::accent()
                                    } else {
                                        Palette::surface_elevated()
                                    })
                                    .border_1()
                                    .border_color(Palette::border())
                                    .cursor_pointer()
                                    .on_mouse_down(
                                        gpui::MouseButton::Left,
                                        cx.listener(move |this, _, cx| {
                                            this.assign_new(scan_id, cx);
                                        }),
                                    )
                                    .child(
                                        div()
                                            .text_color(if is_new {
                                                Palette::background()
                                            } else {
                                                Palette::text_secondary()
                                            })
                                            .text_xs()
                                            .child("New"),
                                    ),
                            ),
                    )
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .w(px(SCAN_HISTORY_WIDTH_PX))
            .h_full()
            .bg(Palette::surface())
            // ── Panel header ────────────────────────────────────────────
            .child(
                div()
                    .flex()
                    .items_center()
                    .w_full()
                    .h(px(36.0))
                    .px(px(10.0))
                    .border_b_1()
                    .border_color(Palette::border())
                    .child(
                        div()
                            .text_color(Palette::text_primary())
                            .text_sm()
                            .child("Scan History"),
                    ),
            )
            // ── Scan list ───────────────────────────────────────────────
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(rows),
            )
            // ── Action buttons ──────────────────────────────────────────
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(8.0))
                    .p(px(8.0))
                    .border_t_1()
                    .border_color(Palette::border())
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .h(px(30.0))
                            .bg(Palette::accent())
                            .hover(|s| s.bg(Palette::accent_hover()))
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, cx| {
                                this.on_compare(cx);
                            }))
                            .child(
                                div()
                                    .text_color(Palette::background())
                                    .text_sm()
                                    .child("Compare"),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .items_center()
                            .justify_center()
                            .h(px(30.0))
                            .bg(Palette::surface_elevated())
                            .hover(|s| s.bg(Palette::surface()))
                            .border_1()
                            .border_color(Palette::border())
                            .rounded(px(4.0))
                            .cursor_pointer()
                            .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, cx| {
                                this.on_delete(cx);
                            }))
                            .child(
                                div()
                                    .text_color(Palette::text_secondary())
                                    .text_sm()
                                    .child("Delete"),
                            ),
                    ),
            )
    }
}