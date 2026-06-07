use gpui::{
    div, px, relative, App, Context, Element, EventEmitter, Focusable, FocusHandle,
    InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled, Window,
};

use crate::theme::current_theme;

/// Events emitted by the title bar.
pub enum TitleBarEvent {
    CloseRequested,
    MinimizeRequested,
    MaximizeRequested,
}

/// Custom title bar with window controls and a centered title.
pub struct TitleBar {
    focus_handle: FocusHandle,
    title: SharedString,
}

impl EventEmitter<TitleBarEvent> for TitleBar {}

impl TitleBar {
    pub fn new(title: impl Into<SharedString>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            title: title.into(),
        }
    }

    pub fn set_title(&mut self, title: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.title = title.into();
        cx.notify();
    }
}

impl Focusable for TitleBar {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = current_theme();
        let title = self.title.clone();

        div()
            .id("title-bar")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(40.0))
            .bg(theme.title_bar_bg)
            .border_b_1()
            .border_color(theme.border)
            // Left: traffic-light placeholder (macOS handles real controls)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .pl(px(16.0))
                    .w(px(80.0))
                    .child(window_control_dot(
                        gpui::red(),
                        "close",
                        cx,
                    ))
                    .child(window_control_dot(
                        gpui::yellow(),
                        "minimize",
                        cx,
                    ))
                    .child(window_control_dot(
                        gpui::green(),
                        "maximize",
                        cx,
                    )),
            )
            // Center: title
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .flex_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::SEMIBOLD)
                            .text_color(theme.title_bar_text)
                            .child(title),
                    ),
            )
            // Right: spacer to balance the left controls
            .child(div().w(px(80.0)))
    }
}

fn window_control_dot(
    color: gpui::Hsla,
    _id: &'static str,
    _cx: &mut Context<TitleBar>,
) -> impl IntoElement {
    div()
        .w(px(12.0))
        .h(px(12.0))
        .rounded_full()
        .bg(color)
}