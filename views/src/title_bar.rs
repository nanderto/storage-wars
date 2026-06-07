use gpui::{
    div, px, svg, AnyElement, AppContext, Div, EventEmitter, FocusHandle, FocusableView,
    IntoElement, Model, ParentElement, Render, SharedString, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::ui_helpers::{colors, font_size, spacing};

/// Actions for window controls
#[derive(Debug, Clone)]
pub enum TitleBarEvent {
    CloseRequested,
    MinimizeRequested,
    MaximizeRequested,
}

/// Custom title bar with window controls and application title
pub struct TitleBar {
    title: SharedString,
    subtitle: Option<SharedString>,
    focus_handle: FocusHandle,
}

impl TitleBar {
    pub fn new(
        title: impl Into<SharedString>,
        subtitle: Option<impl Into<SharedString>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            title: title.into(),
            subtitle: subtitle.map(|s| s.into()),
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_subtitle(&mut self, subtitle: Option<impl Into<SharedString>>, cx: &mut ViewContext<Self>) {
        self.subtitle = subtitle.map(|s| s.into());
        cx.notify();
    }

    fn render_traffic_lights(&self, cx: &mut ViewContext<Self>) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(spacing::xs())
            .child(
                // Close button
                div()
                    .w(px(12.0))
                    .h(px(12.0))
                    .rounded_full()
                    .bg(gpui::rgb(0xff5f57))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.8))
                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, cx| {
                        cx.emit(TitleBarEvent::CloseRequested);
                    })),
            )
            .child(
                // Minimize button
                div()
                    .w(px(12.0))
                    .h(px(12.0))
                    .rounded_full()
                    .bg(gpui::rgb(0xffbd2e))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.8))
                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, cx| {
                        cx.emit(TitleBarEvent::MinimizeRequested);
                    })),
            )
            .child(
                // Maximize button
                div()
                    .w(px(12.0))
                    .h(px(12.0))
                    .rounded_full()
                    .bg(gpui::rgb(0x28c840))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.8))
                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, cx| {
                        cx.emit(TitleBarEvent::MaximizeRequested);
                    })),
            )
    }

    fn render_title_area(&self) -> Div {
        let mut title_row = div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center();

        title_row = title_row.child(
            div()
                .text_color(colors::text())
                .text_size(font_size::md())
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .child(self.title.clone()),
        );

        if let Some(subtitle) = &self.subtitle {
            title_row = title_row.child(
                div()
                    .text_color(colors::subtext())
                    .text_size(font_size::xs())
                    .child(subtitle.clone()),
            );
        }

        title_row
    }
}

impl EventEmitter<TitleBarEvent> for TitleBar {}

impl FocusableView for TitleBar {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TitleBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(40.0))
            .px(spacing::md())
            .bg(colors::title_bar_bg())
            .border_b_1()
            .border_color(colors::border())
            // Allow window dragging from the title bar
            .on_mouse_down(gpui::MouseButton::Left, |_, cx| {
                cx.start_window_move();
            })
            .child(
                // Left: traffic lights
                div()
                    .flex()
                    .items_center()
                    .w(px(80.0))
                    .child(self.render_traffic_lights(cx)),
            )
            .child(
                // Center: title
                div()
                    .flex()
                    .flex_1()
                    .justify_center()
                    .child(self.render_title_area()),
            )
            .child(
                // Right: placeholder for future controls
                div().w(px(80.0)),
            )
    }
}