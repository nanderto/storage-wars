use gpui::{
    div, px, Element, EventEmitter, FocusHandle, FocusableView, IntoElement, ParentElement,
    Render, Styled, ViewContext,
};

use crate::theme::StorageWarsTheme;

/// Primary content area — placeholder for the real application UI.
pub struct MainContent {
    focus_handle: FocusHandle,
}

impl MainContent {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl FocusableView for MainContent {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for MainContent {}

impl Render for MainContent {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<StorageWarsTheme>();
        let surface = theme.surface;
        let fg = theme.foreground;
        let fg_muted = theme.foreground_muted;
        let accent = theme.accent;
        let border = theme.border;

        div()
            .flex()
            .flex_col()
            .flex_1()
            .size_full()
            .bg(surface)
            // Centred welcome / placeholder content
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .flex_1()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_color(accent)
                            .text_size(px(48.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("📦"),
                    )
                    .child(
                        div()
                            .text_color(fg)
                            .text_size(px(24.0))
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .child("Storage Wars"),
                    )
                    .child(
                        div()
                            .text_color(fg_muted)
                            .text_size(px(14.0))
                            .child("Your intelligent storage management companion"),
                    )
                    .child(
                        div()
                            .mt(px(32.0))
                            .px(px(24.0))
                            .py(px(12.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(border)
                            .bg(theme.elevated_surface)
                            .text_color(theme.foreground_subtle)
                            .text_size(px(12.0))
                            .child("Application initialising…"),
                    ),
            )
    }
}