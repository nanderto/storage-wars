//! Shared UI helpers and placeholder content.
//!
//! This module will grow to host reusable components (buttons, cards, etc.)
//! as the application is built out.  For now it provides a simple
//! placeholder that confirms the window is rendering correctly.

use gpui::{div, px, FontWeight, IntoElement, ParentElement, Styled};

use crate::theme::StorageWarsTheme;

/// Renders a centred placeholder that confirms the GPUI window is live.
pub fn placeholder_content(theme: &StorageWarsTheme) -> impl IntoElement {
    let text_color = theme.text;
    let muted = theme.text_muted;
    let accent = theme.accent;

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .flex_1()
        .gap_3()
        .child(
            div()
                .text_color(accent)
                .text_size(px(48.0))
                .font_weight(FontWeight::BOLD)
                .child("📦"),
        )
        .child(
            div()
                .text_color(text_color)
                .text_size(px(28.0))
                .font_weight(FontWeight::SEMIBOLD)
                .child("Storage Wars"),
        )
        .child(
            div()
                .text_color(muted)
                .text_size(px(16.0))
                .child("Your storage unit management hub"),
        )
}