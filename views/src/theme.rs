use gpui::{hsla, rgba, Hsla, Rgba};

/// Application color palette.
pub struct Theme {
    pub background: Hsla,
    pub surface: Hsla,
    pub surface_elevated: Hsla,
    pub border: Hsla,
    pub border_focused: Hsla,

    pub text_primary: Hsla,
    pub text_secondary: Hsla,
    pub text_muted: Hsla,
    pub text_on_accent: Hsla,

    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub accent_pressed: Hsla,

    pub selection_bg: Hsla,
    pub selection_text: Hsla,

    pub size_increased: Hsla,
    pub size_decreased: Hsla,
    pub size_new: Hsla,
    pub size_unchanged: Hsla,

    pub progress_track: Hsla,
    pub progress_fill: Hsla,

    pub title_bar_bg: Hsla,
    pub title_bar_text: Hsla,

    pub button_bg: Hsla,
    pub button_bg_hover: Hsla,
    pub button_bg_pressed: Hsla,
    pub button_text: Hsla,
    pub button_border: Hsla,

    pub scan_history_bg: Hsla,
    pub scan_history_item_hover: Hsla,
    pub scan_history_item_selected: Hsla,

    pub column_header_bg: Hsla,
    pub row_even: Hsla,
    pub row_odd: Hsla,
    pub row_hover: Hsla,
}

impl Theme {
    /// Returns the default dark theme.
    pub fn dark() -> Self {
        Self {
            background: hsla(220.0 / 360.0, 0.14, 0.09, 1.0),
            surface: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
            surface_elevated: hsla(220.0 / 360.0, 0.12, 0.16, 1.0),
            border: hsla(220.0 / 360.0, 0.10, 0.22, 1.0),
            border_focused: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),

            text_primary: hsla(0.0, 0.0, 0.92, 1.0),
            text_secondary: hsla(0.0, 0.0, 0.70, 1.0),
            text_muted: hsla(0.0, 0.0, 0.45, 1.0),
            text_on_accent: hsla(0.0, 0.0, 1.0, 1.0),

            accent: hsla(210.0 / 360.0, 0.80, 0.55, 1.0),
            accent_hover: hsla(210.0 / 360.0, 0.80, 0.62, 1.0),
            accent_pressed: hsla(210.0 / 360.0, 0.80, 0.48, 1.0),

            selection_bg: hsla(210.0 / 360.0, 0.70, 0.40, 0.5),
            selection_text: hsla(0.0, 0.0, 1.0, 1.0),

            size_increased: hsla(0.0 / 360.0, 0.75, 0.55, 1.0),
            size_decreased: hsla(120.0 / 360.0, 0.55, 0.45, 1.0),
            size_new: hsla(45.0 / 360.0, 0.90, 0.55, 1.0),
            size_unchanged: hsla(0.0, 0.0, 0.50, 1.0),

            progress_track: hsla(220.0 / 360.0, 0.10, 0.20, 1.0),
            progress_fill: hsla(210.0 / 360.0, 0.70, 0.50, 1.0),

            title_bar_bg: hsla(220.0 / 360.0, 0.15, 0.10, 1.0),
            title_bar_text: hsla(0.0, 0.0, 0.85, 1.0),

            button_bg: hsla(220.0 / 360.0, 0.12, 0.20, 1.0),
            button_bg_hover: hsla(220.0 / 360.0, 0.12, 0.26, 1.0),
            button_bg_pressed: hsla(220.0 / 360.0, 0.12, 0.16, 1.0),
            button_text: hsla(0.0, 0.0, 0.88, 1.0),
            button_border: hsla(220.0 / 360.0, 0.10, 0.28, 1.0),

            scan_history_bg: hsla(220.0 / 360.0, 0.14, 0.11, 1.0),
            scan_history_item_hover: hsla(220.0 / 360.0, 0.12, 0.18, 1.0),
            scan_history_item_selected: hsla(210.0 / 360.0, 0.60, 0.30, 0.6),

            column_header_bg: hsla(220.0 / 360.0, 0.13, 0.14, 1.0),
            row_even: hsla(220.0 / 360.0, 0.13, 0.12, 1.0),
            row_odd: hsla(220.0 / 360.0, 0.13, 0.10, 1.0),
            row_hover: hsla(220.0 / 360.0, 0.12, 0.18, 0.8),
        }
    }

    /// Returns the default light theme.
    pub fn light() -> Self {
        Self {
            background: hsla(0.0, 0.0, 0.97, 1.0),
            surface: hsla(0.0, 0.0, 1.0, 1.0),
            surface_elevated: hsla(0.0, 0.0, 0.96, 1.0),
            border: hsla(0.0, 0.0, 0.85, 1.0),
            border_focused: hsla(210.0 / 360.0, 0.80, 0.50, 1.0),

            text_primary: hsla(0.0, 0.0, 0.10, 1.0),
            text_secondary: hsla(0.0, 0.0, 0.35, 1.0),
            text_muted: hsla(0.0, 0.0, 0.55, 1.0),
            text_on_accent: hsla(0.0, 0.0, 1.0, 1.0),

            accent: hsla(210.0 / 360.0, 0.80, 0.50, 1.0),
            accent_hover: hsla(210.0 / 360.0, 0.80, 0.44, 1.0),
            accent_pressed: hsla(210.0 / 360.0, 0.80, 0.38, 1.0),

            selection_bg: hsla(210.0 / 360.0, 0.70, 0.70, 0.4),
            selection_text: hsla(0.0, 0.0, 0.05, 1.0),

            size_increased: hsla(0.0 / 360.0, 0.70, 0.45, 1.0),
            size_decreased: hsla(120.0 / 360.0, 0.50, 0.38, 1.0),
            size_new: hsla(38.0 / 360.0, 0.90, 0.45, 1.0),
            size_unchanged: hsla(0.0, 0.0, 0.55, 1.0),

            progress_track: hsla(0.0, 0.0, 0.88, 1.0),
            progress_fill: hsla(210.0 / 360.0, 0.70, 0.50, 1.0),

            title_bar_bg: hsla(0.0, 0.0, 0.95, 1.0),
            title_bar_text: hsla(0.0, 0.0, 0.15, 1.0),

            button_bg: hsla(0.0, 0.0, 0.92, 1.0),
            button_bg_hover: hsla(0.0, 0.0, 0.86, 1.0),
            button_bg_pressed: hsla(0.0, 0.0, 0.80, 1.0),
            button_text: hsla(0.0, 0.0, 0.12, 1.0),
            button_border: hsla(0.0, 0.0, 0.78, 1.0),

            scan_history_bg: hsla(0.0, 0.0, 0.96, 1.0),
            scan_history_item_hover: hsla(0.0, 0.0, 0.90, 1.0),
            scan_history_item_selected: hsla(210.0 / 360.0, 0.60, 0.80, 0.5),

            column_header_bg: hsla(0.0, 0.0, 0.93, 1.0),
            row_even: hsla(0.0, 0.0, 1.0, 1.0),
            row_odd: hsla(0.0, 0.0, 0.97, 1.0),
            row_hover: hsla(210.0 / 360.0, 0.50, 0.92, 0.8),
        }
    }
}

/// Global theme accessor — returns the dark theme by default.
pub fn current_theme() -> Theme {
    Theme::dark()
}