//! Custom title bar component rendered at the top of the main window.
//!
//! On macOS the native traffic-light buttons are positioned by GPUI; this
//! component fills the remaining space with the application name and any
//! future toolbar actions.

use gpui::{
    div, px, AnyElement, Element, IntoElement, ParentElement, RenderOnce, Styled,
};

use crate::theme::StorageWarsTheme;

/// Height of the title bar in logical pixels.
pub const TITLE_BAR_HEIGHT: f32 = 36.0;

/// Left padding that clears the macOS traffic-light buttons.
const TRAFFIC_LIGHT_CLEARANCE: f32 = 72.0;

/// The custom title bar element.
#[derive(IntoElement)]
pub struct TitleBar {
    theme: StorageWarsTheme,
}

impl TitleBar {
    /// Creates a new [`TitleBar`] with the given theme.
    pub fn new(theme: StorageWarsTheme) -> Self {
        Self { theme }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        div()
            // Allow the OS / GPUI to use this region for window dragging.
            .id("title-bar")
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(TITLE_BAR_HEIGHT))
            .bg(self.theme.title_bar_background)
            .border_b_1()
            .border_color(self.theme.border)
            // Pad left to avoid overlapping the macOS traffic-light buttons.
            .pl(px(TRAFFIC_LIGHT_CLEARANCE))
            .pr(px(16.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(app_icon())
                    .child(app_title(&self.theme)),
            )
    }
}

/// Renders a simple coloured square as a placeholder application icon.
fn app_icon() -> impl IntoElement {
    div()
        .w(px(16.0))
        .h(px(16.0))
        .rounded(px(3.0))
        .bg(gpui::hsla(217.0 / 360.0, 0.91, 0.60, 1.0))
}

/// Renders the application name label.
fn app_title(theme: &StorageWarsTheme) -> impl IntoElement {
    div()
        .text_sm()
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .text_color(theme.title_bar_foreground)
        .child("Storage Wars")
}