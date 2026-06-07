//! Custom title bar component.
//!
//! Renders the application name and any global toolbar actions
//! inside the OS-native title bar area.

use gpui::{div, px, IntoElement, ParentElement, Render, Styled, ViewContext};

use crate::theme::StorageWarsTheme;

/// A thin custom title bar rendered inside the transparent OS title bar region.
pub struct TitleBar {
    theme: StorageWarsTheme,
}

impl TitleBar {
    /// Creates a new [`TitleBar`] using the provided theme.
    pub fn new(theme: StorageWarsTheme) -> Self {
        Self { theme }
    }
}

impl Render for TitleBar {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text_color = self.theme.text;
        let bg = self.theme.surface;

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(40.0))
            .bg(bg)
            .px_4()
            .child(
                div()
                    .text_color(text_color)
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child("Storage Wars"),
            )
    }
}