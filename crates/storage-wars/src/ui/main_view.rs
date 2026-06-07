use gpui::{
    div, px, Context, IntoElement, ParentElement, Render, Styled, Window,
};

/// The primary content area of the Storage Wars application.
///
/// This view will host the storage-unit browser, detail panels, and
/// action toolbars once the sub-modules are implemented.
pub struct MainView;

impl MainView {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .w_full()
            .p(px(24.0))
            .bg(gpui::rgb(0x1a1a2e))
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
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(gpui::rgb(0xe94560))
                            .child("Storage Wars"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(gpui::rgb(0xa0a0b0))
                            .child("Your storage unit management dashboard is loading…"),
                    ),
            )
    }
}