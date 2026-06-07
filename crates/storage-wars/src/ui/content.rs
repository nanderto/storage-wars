//! Main content area rendered below the title bar.
//!
//! This is the primary workspace of the application. Initially it shows a
//! welcome / placeholder state; real content panels will be added as the
//! application grows.

use gpui::{
    div, px, Element, IntoElement, ParentElement, RenderOnce, Styled,
};

use crate::theme::StorageWarsTheme;

/// The main content area of the application window.
#[derive(IntoElement)]
pub struct ContentArea {
    theme: StorageWarsTheme,
}

impl ContentArea {
    /// Creates a new [`ContentArea`] with the given theme.
    pub fn new(theme: StorageWarsTheme) -> Self {
        Self { theme }
    }
}

impl RenderOnce for ContentArea {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .size_full()
            .bg(self.theme.background)
            .child(welcome_screen(&self.theme))
    }
}

/// Renders a centered welcome message for the initial empty state.
fn welcome_screen(theme: &StorageWarsTheme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .items_center()
        .justify_center()
        .gap(px(12.0))
        .child(
            div()
                .text_xl()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme.foreground)
                .child("Storage Wars"),
        )
        .child(
            div()
                .text_sm()
                .text_color(theme.foreground_muted)
                .child("Your storage unit management dashboard"),
        )
        .child(
            div()
                .mt(px(24.0))
                .px(px(20.0))
                .py(px(10.0))
                .rounded(px(6.0))
                .bg(theme.accent)
                .text_color(theme.accent_foreground)
                .text_sm()
                .font_weight(gpui::FontWeight::MEDIUM)
                .child("Get Started"),
        )
}