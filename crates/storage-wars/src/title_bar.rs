use gpui::{
    div, px, AnyElement, Element, EventEmitter, FocusHandle, FocusableView, IntoElement,
    ParentElement, Render, Styled, View, ViewContext, VisualContext,
};

use crate::theme::StorageWarsTheme;

/// Custom title bar rendered inside the GPUI window chrome.
pub struct TitleBar {
    focus_handle: FocusHandle,
}

impl TitleBar {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for TitleBar {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for TitleBar {}

impl Render for TitleBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<StorageWarsTheme>();
        let bg = theme.title_bar_background;
        let fg = theme.foreground;
        let accent = theme.accent;
        let border = theme.border_subtle;

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(40.0))
            .bg(bg)
            .border_b_1()
            .border_color(border)
            // Left: traffic-light spacer (macOS) + app name
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    // Reserve space for macOS traffic lights (80 px)
                    .pl(px(80.0))
                    .child(
                        div()
                            .text_color(accent)
                            .text_size(px(13.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("⚡ Storage Wars"),
                    ),
            )
            // Right: placeholder for future toolbar actions
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .pr(px(16.0))
                    .child(
                        div()
                            .text_color(theme.foreground_subtle)
                            .text_size(px(11.0))
                            .child("v0.1.0"),
                    ),
            )
    }
}