use gpui::{Global, Hsla, hsla};

/// Application-wide theme token set.
#[derive(Debug, Clone, Copy)]
pub struct StorageWarsTheme {
    // Backgrounds
    pub background: Hsla,
    pub surface: Hsla,
    pub elevated_surface: Hsla,
    pub title_bar_background: Hsla,

    // Foregrounds
    pub foreground: Hsla,
    pub foreground_muted: Hsla,
    pub foreground_subtle: Hsla,

    // Accents
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub accent_active: Hsla,

    // Borders
    pub border: Hsla,
    pub border_subtle: Hsla,

    // Status colours
    pub success: Hsla,
    pub warning: Hsla,
    pub error: Hsla,
    pub info: Hsla,
}

impl StorageWarsTheme {
    /// Construct the canonical dark theme for Storage Wars.
    pub fn dark() -> Self {
        Self {
            // Backgrounds — deep charcoal palette
            background: hsla(220.0 / 360.0, 0.13, 0.09, 1.0),
            surface: hsla(220.0 / 360.0, 0.12, 0.12, 1.0),
            elevated_surface: hsla(220.0 / 360.0, 0.11, 0.16, 1.0),
            title_bar_background: hsla(220.0 / 360.0, 0.14, 0.08, 1.0),

            // Foregrounds
            foreground: hsla(220.0 / 360.0, 0.10, 0.92, 1.0),
            foreground_muted: hsla(220.0 / 360.0, 0.08, 0.65, 1.0),
            foreground_subtle: hsla(220.0 / 360.0, 0.06, 0.42, 1.0),

            // Accent — amber / gold to match the "storage wars" theme
            accent: hsla(38.0 / 360.0, 0.92, 0.55, 1.0),
            accent_hover: hsla(38.0 / 360.0, 0.92, 0.62, 1.0),
            accent_active: hsla(38.0 / 360.0, 0.92, 0.48, 1.0),

            // Borders
            border: hsla(220.0 / 360.0, 0.10, 0.22, 1.0),
            border_subtle: hsla(220.0 / 360.0, 0.08, 0.16, 1.0),

            // Status
            success: hsla(142.0 / 360.0, 0.69, 0.58, 1.0),
            warning: hsla(38.0 / 360.0, 0.92, 0.55, 1.0),
            error: hsla(0.0 / 360.0, 0.72, 0.51, 1.0),
            info: hsla(210.0 / 360.0, 0.80, 0.60, 1.0),
        }
    }
}

impl Global for StorageWarsTheme {}