use gpui::AppContext;
use log::debug;

/// Dark colour palette used throughout the application.
pub struct DarkTheme {
    pub background: gpui::Rgba,
    pub surface: gpui::Rgba,
    pub surface_elevated: gpui::Rgba,
    pub border: gpui::Rgba,
    pub text_primary: gpui::Rgba,
    pub text_secondary: gpui::Rgba,
    pub text_muted: gpui::Rgba,
    pub accent: gpui::Rgba,
    pub accent_hover: gpui::Rgba,
    pub danger: gpui::Rgba,
    pub success: gpui::Rgba,
    pub warning: gpui::Rgba,
}

impl Default for DarkTheme {
    fn default() -> Self {
        Self {
            background: gpui::rgb(0x1a1a2e),
            surface: gpui::rgb(0x16213e),
            surface_elevated: gpui::rgb(0x0f3460),
            border: gpui::rgb(0x2a2a4a),
            text_primary: gpui::rgb(0xe0e0e0),
            text_secondary: gpui::rgb(0xa0a0b0),
            text_muted: gpui::rgb(0x606070),
            accent: gpui::rgb(0xe94560),
            accent_hover: gpui::rgb(0xff6b7a),
            danger: gpui::rgb(0xff4757),
            success: gpui::rgb(0x2ed573),
            warning: gpui::rgb(0xffa502),
        }
    }
}

/// Apply the dark theme to the GPUI application context.
///
/// This is called once during startup before any windows are opened.
pub fn apply_dark_theme(_cx: &mut AppContext) {
    debug!("Applying dark theme");
    // Theme tokens are accessed via `DarkTheme::default()` throughout the
    // component tree. Future iterations can store the active theme in a
    // global GPUI model here.
}