use gpui::{
    div, px, App, Context, Element, EventEmitter, Focusable, FocusHandle, InteractiveElement,
    IntoElement, ParentElement, Render, SharedString, Styled, Window,
};
use uuid::Uuid;

use crate::theme::current_theme;
use crate::types::{ScanEntry, ScanRole};

/// Events emitted by the scan history panel.
pub enum ScanHistoryEvent {
    BaseSelected(Uuid),
    NewSelected(Uuid),
    CompareRequested,
    DeleteRequested(Uuid),
}

impl EventEmitter<ScanHistoryEvent> for ScanHistory {}

/// A 280px-wide focusable panel showing scan history with Base/New selection.
pub struct ScanHistory {
    focus_handle: FocusHandle,
    scans: Vec<ScanEntry>,
    base_scan_id: Option<Uuid>,
    new_scan_id: Option<Uuid>,
    hovered_id: Option<Uuid>,
    pending_role: ScanRole,
}

impl ScanHistory {
    pub fn new(
        scans: Vec<ScanEntry>,
        base_scan_id: Option<Uuid>,
        new_scan_id: Option<Uuid>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            scans,
            base_scan_id,
            new_scan_id,
            hovered_id: None,
            pending_role: ScanRole::Base,
        }
    }

    pub fn update_scans(
        &mut self,
        scans: Vec<ScanEntry>,
        base_id: Option<Uuid>,
        new_id: Option<Uuid>,
        cx: &mut Context<Self>,
    ) {
        self.scans = scans;
        self.base_scan_id = base_id;
        self.new_scan_id = new_id;
        cx.notify();
    }

    fn format_date(entry: &ScanEntry) -> String {
        entry.scanned_at.format("%Y-%m-%d %H:%M").to_string()
    }

    fn format_size(bytes: u64) -> String {
        let gb = bytes as f64 / 1_073_741_824.0;
        if gb >= 1.0 {
            format!("{gb:.1} GB")
        } else {
            let mb = bytes as f64 / 1_048_576.0;
            format!("{mb:.0} MB")
        }
    }
}

impl Focusable for ScanHistory {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ScanHistory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = current_theme();
        let is_focused = self.focus_handle.is_focused(cx);

        let base_id = self.base_scan_id;
        let new_id = self.new_scan_id;
        let can_compare = base_id.is_some() && new_id.is_some();

        let border_color = if is_focused {
            theme.border_focused
        } else {
            theme.border
        };

        div()
            .id("scan-history")
            .flex()
            .flex_col()
            .w(px(280.0))
            .h_full()
            .bg(theme.scan_history_bg)
            .border_r_1()
            .border_color(border_color)
            // Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(px(40.0))
                    .px(px(12.0))
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(theme.text_primary)
                            .child("Scan History"),
                    ),
            )
            // Scan list
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(self.scans.iter().enumerate().map(|(i, scan)| {
                        let scan_id = scan.id;
                        let is_base = base_id == Some(scan_id);
                        let is_new = new_id == Some(scan_id);
                        let label = scan.label.clone();
                        let date_str = Self::format_date(scan);
                        let size_str = Self::format_size(scan.total_bytes);

                        let bg = if is_base || is_new {
                            theme.scan_history_item_selected
                        } else {
                            theme.scan_history_bg
                        };

                        div()
                            .id(("scan-item", i))
                            .flex()
                            .flex_col()
                            .px(px(12.0))
                            .py(px(8.0))
                            .border_b_1()
                            .border_color(theme.border)
                            .bg(bg)
                            .hover(|s| s.bg(theme.scan_history_item_hover))
                            .cursor_pointer()
                            .on_click(cx.listener(move |this, _, _window, cx| {
                                match this.pending_role {
                                    ScanRole::Base => {
                                        this.base_scan_id = Some(scan_id);
                                        this.pending_role = ScanRole::New;
                                        cx.emit(ScanHistoryEvent::BaseSelected(scan_id));
                                    }
                                    ScanRole::New => {
                                        this.new_scan_id = Some(scan_id);
                                        this.pending_role = ScanRole::Base;
                                        cx.emit(ScanHistoryEvent::NewSelected(scan_id));
                                    }
                                }
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(gpui::FontWeight::MEDIUM)
                                            .text_color(theme.text_primary)
                                            .child(SharedString::from(label)),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .gap(px(4.0))
                                            .when(is_base, |el| {
                                                el.child(role_badge("BASE", theme.size_decreased))
                                            })
                                            .when(is_new, |el| {
                                                el.child(role_badge("NEW", theme.accent))
                                            }),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .mt(px(2.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(SharedString::from(date_str)),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .child(SharedString::from(size_str)),
                                    ),
                            )
                    })),
            )
            // Footer buttons
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .border_t_1()
                    .border_color(theme.border)
                    .child(action_button(
                        "Compare",
                        can_compare,
                        theme.accent,
                        theme.text_on_accent,
                        cx,
                        |_this, _window, cx| {
                            cx.emit(ScanHistoryEvent::CompareRequested);
                        },
                    ))
                    .child(action_button(
                        "Delete",
                        new_id.is_some(),
                        theme.size_increased,
                        theme.text_on_accent,
                        cx,
                        move |_this, _window, cx| {
                            if let Some(id) = new_id {
                                cx.emit(ScanHistoryEvent::DeleteRequested(id));
                            }
                        },
                    )),
            )
    }
}

fn role_badge(label: &'static str, color: gpui::Hsla) -> impl IntoElement {
    div()
        .px(px(4.0))
        .py(px(1.0))
        .rounded(px(3.0))
        .bg(color)
        .text_xs()
        .font_weight(gpui::FontWeight::BOLD)
        .text_color(gpui::white())
        .child(label)
}

fn action_button(
    label: &'static str,
    enabled: bool,
    bg: gpui::Hsla,
    text_color: gpui::Hsla,
    cx: &mut Context<ScanHistory>,
    handler: impl Fn(&mut ScanHistory, &mut Window, &mut Context<ScanHistory>) + 'static,
) -> impl IntoElement {
    let theme = current_theme();
    let opacity = if enabled { 1.0 } else { 0.4 };

    div()
        .id(label)
        .flex()
        .items_center()
        .justify_center()
        .h(px(30.0))
        .px(px(12.0))
        .rounded(px(5.0))
        .bg(if enabled { bg } else { theme.button_bg })
        .border_1()
        .border_color(if enabled { bg } else { theme.button_border })
        .cursor_pointer()
        .when(enabled, |el| {
            el.on_click(cx.listener(move |this, _, window, cx| {
                handler(this, window, cx);
            }))
        })
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .text_color(if enabled { text_color } else { theme.text_muted })
                .child(label),
        )
}