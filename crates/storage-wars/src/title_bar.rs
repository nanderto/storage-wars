use gpui::{div, px, Entity, IntoElement, ParentElement, Render, SharedString, Styled, ViewContext};

use crate::theme::StorageWarsTheme;

/// Custom title-bar component rendered inside the transparent native title bar.
pub struct TitleBar {
    title: SharedString,
    theme: StorageWarsTheme,
}

impl TitleBar {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            theme: StorageWarsTheme::dark(),
        }
    }
}

impl Render for TitleBar {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = self.theme.clone();

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .w_full()
            .h(px(40.0))
            .bg(theme.title_bar_background)
            .border_b_1()
            .border_color(theme.border)
            // Leave space on the left for macOS traffic lights (≈ 72 px).
            .pl(px(72.0))
            .child(
                div()
                    .text_sm()
                    .text_color(theme.muted)
                    .child(self.title.clone()),
            )
    }
}