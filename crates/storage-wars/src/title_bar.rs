use gpui::{
    div, px, App, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
};

/// Custom title bar rendered inside the GPUI window.
///
/// Replaces the OS-native chrome so we can apply the dark theme consistently
/// across all platforms.
pub struct TitleBar {
    title: String,
}

impl TitleBar {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            title: "Storage Wars".to_string(),
        }
    }
}

impl Render for TitleBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(40.0))
            .px(px(16.0))
            .bg(gpui::rgb(0x16213e))
            .border_b_1()
            .border_color(gpui::rgb(0x2a2a4a))
            // Window drag region — the entire title bar can be used to move the window.
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        // Application icon placeholder
                        div()
                            .w(px(20.0))
                            .h(px(20.0))
                            .rounded(px(4.0))
                            .bg(gpui::rgb(0xe94560)),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(gpui::rgb(0xe0e0e0))
                            .child(self.title.clone()),
                    ),
            )
            .child(
                // Window controls placeholder (close / minimise / maximise)
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(window_control_button(gpui::rgb(0xff5f57)))
                    .child(window_control_button(gpui::rgb(0xffbd2e)))
                    .child(window_control_button(gpui::rgb(0x28c840))),
            )
    }
}

/// Render a single circular window-control button.
fn window_control_button(color: gpui::Rgba) -> impl IntoElement {
    div()
        .w(px(12.0))
        .h(px(12.0))
        .rounded_full()
        .bg(color)
        .cursor_pointer()
}