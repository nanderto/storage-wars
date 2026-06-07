use gpui::{div, px, App, Context, Element, IntoElement, ParentElement, Styled};

use crate::theme::StorageWarsTheme;

/// Thin custom title-bar rendered inside the GPUI window.
///
/// On macOS the native traffic-light buttons are positioned by the
/// [`TitlebarOptions`] in `main.rs`; this bar provides the visual
/// background strip and application title text.
pub struct TitleBar;

impl TitleBar {
    /// Return a renderable element for the title bar.
    pub fn render(theme: &StorageWarsTheme, _cx: &mut Context<impl gpui::EventEmitter<()> + 'static>) -> impl IntoElement {
        let bg = theme.title_bar_background;
        let fg = theme.title_bar_foreground;
        let border = theme.border;

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .w_full()
            .h(px(40.0))
            .bg(bg)
            .border_b_1()
            .border_color(border)
            // Leave room for macOS traffic lights on the left.
            .pl(px(80.0))
            .pr(px(16.0))
            .child(
                div()
                    .text_sm()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child("Storage Wars"),
            )
    }
}