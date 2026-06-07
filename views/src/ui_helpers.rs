use gpui::{
    div, px, rgb, rgba, AbsoluteLength, AnyElement, Div, ElementId, Hsla, IntoElement,
    ParentElement, Pixels, Rgba, SharedString, Styled, WindowContext,
};

use crate::types::SizeChange;

/// Standard color palette for the application
pub mod colors {
    use gpui::{rgb, Hsla, Rgba};

    pub fn background() -> Hsla {
        rgb(0x1e1e2e).into()
    }

    pub fn surface() -> Hsla {
        rgb(0x313244).into()
    }

    pub fn overlay() -> Hsla {
        rgb(0x45475a).into()
    }

    pub fn text() -> Hsla {
        rgb(0xcdd6f4).into()
    }

    pub fn subtext() -> Hsla {
        rgb(0xa6adc8).into()
    }

    pub fn muted() -> Hsla {
        rgb(0x6c7086).into()
    }

    pub fn accent() -> Hsla {
        rgb(0x89b4fa).into()
    }

    pub fn green() -> Hsla {
        rgb(0xa6e3a1).into()
    }

    pub fn red() -> Hsla {
        rgb(0xf38ba8).into()
    }

    pub fn yellow() -> Hsla {
        rgb(0xf9e2af).into()
    }

    pub fn blue() -> Hsla {
        rgb(0x89b4fa).into()
    }

    pub fn mauve() -> Hsla {
        rgb(0xcba6f7).into()
    }

    pub fn border() -> Hsla {
        rgb(0x45475a).into()
    }

    pub fn title_bar_bg() -> Hsla {
        rgb(0x181825).into()
    }

    /// Color for a size change indicator
    pub fn for_size_change(change: crate::types::SizeChange) -> Hsla {
        match change {
            crate::types::SizeChange::Increased => red(),
            crate::types::SizeChange::Decreased => green(),
            crate::types::SizeChange::Unchanged => subtext(),
            crate::types::SizeChange::New => blue(),
            crate::types::SizeChange::Deleted => muted(),
        }
    }
}

/// Standard spacing constants
pub mod spacing {
    use gpui::{px, Pixels};

    pub fn xs() -> Pixels {
        px(4.0)
    }

    pub fn sm() -> Pixels {
        px(8.0)
    }

    pub fn md() -> Pixels {
        px(12.0)
    }

    pub fn lg() -> Pixels {
        px(16.0)
    }

    pub fn xl() -> Pixels {
        px(24.0)
    }

    pub fn xxl() -> Pixels {
        px(32.0)
    }

    /// Indentation per depth level in the tree view
    pub fn tree_indent() -> Pixels {
        px(16.0)
    }
}

/// Standard font sizes
pub mod font_size {
    use gpui::{px, Pixels};

    pub fn xs() -> Pixels {
        px(11.0)
    }

    pub fn sm() -> Pixels {
        px(12.0)
    }

    pub fn md() -> Pixels {
        px(13.0)
    }

    pub fn lg() -> Pixels {
        px(14.0)
    }

    pub fn xl() -> Pixels {
        px(16.0)
    }
}

/// Build a horizontal divider element
pub fn divider() -> Div {
    div()
        .w_full()
        .h(px(1.0))
        .bg(colors::border())
        .my(spacing::xs())
}

/// Build a styled button
pub fn button(label: impl Into<SharedString>) -> Div {
    let label_str = label.into();
    div()
        .px(spacing::md())
        .py(spacing::xs())
        .rounded(px(6.0))
        .bg(colors::surface())
        .text_color(colors::text())
        .text_size(font_size::sm())
        .cursor_pointer()
        .hover(|s| s.bg(colors::overlay()))
        .child(label_str)
}

/// Build a primary (accent-colored) button
pub fn primary_button(label: impl Into<SharedString>) -> Div {
    let label_str = label.into();
    div()
        .px(spacing::md())
        .py(spacing::xs())
        .rounded(px(6.0))
        .bg(colors::accent())
        .text_color(colors::background())
        .text_size(font_size::sm())
        .cursor_pointer()
        .hover(|s| s.opacity(0.85))
        .child(label_str)
}

/// Build a progress bar element
pub fn progress_bar(fraction: f32, change: SizeChange) -> Div {
    let clamped = fraction.clamp(0.0, 1.0);
    let bar_color = colors::for_size_change(change);

    div()
        .w_full()
        .h(px(6.0))
        .rounded(px(3.0))
        .bg(colors::overlay())
        .child(
            div()
                .h_full()
                .rounded(px(3.0))
                .bg(bar_color)
                .w(gpui::relative(clamped)),
        )
}

/// Build a label with muted styling
pub fn muted_label(text: impl Into<SharedString>) -> Div {
    div()
        .text_color(colors::muted())
        .text_size(font_size::xs())
        .child(text.into())
}

/// Build a section header
pub fn section_header(text: impl Into<SharedString>) -> Div {
    div()
        .text_color(colors::subtext())
        .text_size(font_size::xs())
        .font_weight(gpui::FontWeight::SEMIBOLD)
        .px(spacing::sm())
        .py(spacing::xs())
        .child(text.into())
}