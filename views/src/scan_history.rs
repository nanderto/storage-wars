use gpui::{
    div, px, AppContext, Div, EventEmitter, FocusHandle, FocusableView, IntoElement,
    ParentElement, Render, SharedString, Styled, View, ViewContext, VisualContext, WindowContext,
};
use uuid::Uuid;

use crate::types::ScanSnapshot;
use crate::ui_helpers::{colors, font_size, spacing, button, primary_button, divider, section_header};

/// Panel width constant
pub const SCAN_HISTORY_WIDTH: f32 = 280.0;

/// Events emitted by ScanHistory
#[derive(Debug, Clone)]
pub enum ScanHistoryEvent {
    BaseSelected(Uuid),
    NewSelected(Uuid),
    CompareRequested,
    DeleteRequested(Uuid),
}

/// Selection role for a scan entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanRole {
    Base,
    New,
    None,
}

/// Focusable scan history panel (280px wide)
pub struct ScanHistory {
    scans: Vec<ScanSnapshot>,
    base_id: Option<Uuid>,
    new_id: Option<Uuid>,
    focus_handle: FocusHandle,
}

impl ScanHistory {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            scans: Vec::new(),
            base_id: None,
            new_id: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_scans(&mut self, scans: Vec<ScanSnapshot>, cx: &mut ViewContext<Self>) {
        self.scans = scans;
        cx.notify();
    }

    pub fn set_base_id(&mut self, id: Option<Uuid>, cx: &mut ViewContext<Self>) {
        self.base_id = id;
        cx.notify();
    }

    pub fn set_new_id(&mut self, id: Option<Uuid>, cx: &mut ViewContext<Self>) {
        self.new_id = id;
        cx.notify();
    }

    fn scan_role(&self, id: Uuid) -> ScanRole {
        if self.base_id == Some(id) {
            ScanRole::Base
        } else if self.new_id == Some(id) {
            ScanRole::New
        } else {
            ScanRole::None
        }
    }

    fn render_scan_entry(&self, scan: &ScanSnapshot, cx: &mut ViewContext<Self>) -> Div {
        let role = self.scan_role(scan.id);
        let scan_id = scan.id;
        let label = scan.label.clone();
        let date_str = scan.scanned_at.format("%Y-%m-%d %H:%M").to_string();
        let size_str = crate::types::format_bytes(scan.total_bytes);

        let role_badge = match role {
            ScanRole::Base => Some(
                div()
                    .px(spacing::xs())
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .bg(colors::mauve())
                    .text_color(colors::background())
                    .text_size(font_size::xs())
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("BASE"),
            ),
            ScanRole::New => Some(
                div()
                    .px(spacing::xs())
                    .py(px(2.0))
                    .rounded(px(4.0))
                    .bg(colors::green())
                    .text_color(colors::background())
                    .text_size(font_size::xs())
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("NEW"),
            ),
            ScanRole::None => None,
        };

        let is_highlighted = role != ScanRole::None;

        div()
            .flex()
            .flex_col()
            .w_full()
            .px(spacing::sm())
            .py(spacing::xs())
            .rounded(px(6.0))
            .mb(spacing::xs())
            .bg(if is_highlighted {
                colors::overlay()
            } else {
                colors::surface()
            })
            .border_1()
            .border_color(match role {
                ScanRole::Base => colors::mauve(),
                ScanRole::New => colors::green(),
                ScanRole::None => colors::border(),
            })
            .cursor_pointer()
            .hover(|s| s.bg(colors::overlay()))
            .child(
                // Header row: label + badge
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .mb(px(2.0))
                    .child(
                        div()
                            .text_color(colors::text())
                            .text_size(font_size::sm())
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .overflow_hidden()
                            .child(SharedString::from(label)),
                    )
                    .child(match role_badge {
                        Some(badge) => badge,
                        None => div(),
                    }),
            )
            .child(
                // Date and size row
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_color(colors::muted())
                            .text_size(font_size::xs())
                            .child(SharedString::from(date_str)),
                    )
                    .child(
                        div()
                            .text_color(colors::subtext())
                            .text_size(font_size::xs())
                            .child(SharedString::from(size_str)),
                    ),
            )
            .child(
                // Action buttons row
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(spacing::xs())
                    .mt(spacing::xs())
                    .child(
                        div()
                            .px(spacing::xs())
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .bg(colors::overlay())
                            .text_color(colors::mauve())
                            .text_size(font_size::xs())
                            .cursor_pointer()
                            .hover(|s| s.bg(colors::border()))
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |_, _, cx| {
                                    cx.emit(ScanHistoryEvent::BaseSelected(scan_id));
                                }),
                            )
                            .child("Set Base"),
                    )
                    .child(
                        div()
                            .px(spacing::xs())
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .bg(colors::overlay())
                            .text_color(colors::green())
                            .text_size(font_size::xs())
                            .cursor_pointer()
                            .hover(|s| s.bg(colors::border()))
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |_, _, cx| {
                                    cx.emit(ScanHistoryEvent::NewSelected(scan_id));
                                }),
                            )
                            .child("Set New"),
                    )
                    .child(
                        div()
                            .px(spacing::xs())
                            .py(px(2.0))
                            .rounded(px(4.0))
                            .bg(colors::overlay())
                            .text_color(colors::red())
                            .text_size(font_size::xs())
                            .cursor_pointer()
                            .hover(|s| s.bg(colors::border()))
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |_, _, cx| {
                                    cx.emit(ScanHistoryEvent::DeleteRequested(scan_id));
                                }),
                            )
                            .child("Delete"),
                    ),
            )
    }

    fn render_compare_button(&self, cx: &mut ViewContext<Self>) -> Div {
        let can_compare = self.base_id.is_some() && self.new_id.is_some();

        div()
            .w_full()
            .px(spacing::sm())
            .py(spacing::xs())
            .child(
                div()
                    .w_full()
                    .px(spacing::md())
                    .py(spacing::sm())
                    .rounded(px(6.0))
                    .bg(if can_compare {
                        colors::accent()
                    } else {
                        colors::overlay()
                    })
                    .text_color(if can_compare {
                        colors::background()
                    } else {
                        colors::muted()
                    })
                    .text_size(font_size::sm())
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_align(gpui::TextAlign::Center)
                    .cursor_pointer()
                    .hover(|s| if can_compare { s.opacity(0.85) } else { s })
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(move |_, _, cx| {
                            cx.emit(ScanHistoryEvent::CompareRequested);
                        }),
                    )
                    .child("Compare Scans"),
            )
    }
}

impl EventEmitter<ScanHistoryEvent> for ScanHistory {}

impl FocusableView for ScanHistory {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ScanHistory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let scans = self.scans.clone();
        let is_focused = self.focus_handle.is_focused(cx);

        div()
            .flex()
            .flex_col()
            .w(px(SCAN_HISTORY_WIDTH))
            .flex_shrink_0()
            .h_full()
            .bg(colors::surface())
            .border_l_1()
            .border_color(if is_focused {
                colors::accent()
            } else {
                colors::border()
            })
            .track_focus(&self.focus_handle)
            .child(section_header("Scan History"))
            .child(divider())
            .child(
                // Scan list
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .px(spacing::xs())
                    .py(spacing::xs())
                    .when(scans.is_empty(), |d| {
                        d.child(
                            div()
                                .flex()
                                .flex_1()
                                .items_center()
                                .justify_center()
                                .py(spacing::xl())
                                .child(
                                    div()
                                        .text_color(colors::muted())
                                        .text_size(font_size::sm())
                                        .child("No scans yet"),
                                ),
                        )
                    })
                    .when(!scans.is_empty(), |mut d| {
                        for scan in &scans {
                            d = d.child(self.render_scan_entry(scan, cx));
                        }
                        d
                    }),
            )
            .child(divider())
            .child(self.render_compare_button(cx))
    }
}