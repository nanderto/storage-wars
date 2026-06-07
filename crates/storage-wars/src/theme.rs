use gpui::{Hsla, Rgba};

/// Dark theme palette for Storage Wars.
#[derive(Debug, Clone, Copy)]
pub struct StorageWarsTheme {
    /// Main window background.
    pub background: Hsla,
    /// Primary foreground / text colour.
    pub foreground: Hsla,
    /// Title-bar background (slightly lighter than window bg).
    pub title_bar_background: Hsla,
    /// Title-bar text / icon colour.
    pub title_bar_foreground: Hsla,
    /// Accent colour used for interactive elements.
    pub accent: Hsla,
    /// Subtle border colour.
    pub border: Hsla,
    /// Surface colour for cards / panels.
    pub surface: Hsla,
    /// Muted text colour.
    pub muted: Hsla,
}

impl StorageWarsTheme {
    /// Construct the canonical dark theme.
    pub fn dark() -> Self {
        Self {
            background: rgba_to_hsla(0x0d0d0dff),
            foreground: rgba_to_hsla(0xe8e8e8ff),
            title_bar_background: rgba_to_hsla(0x141414ff),
            title_bar_foreground: rgba_to_hsla(0xd0d0d0ff),
            accent: rgba_to_hsla(0x4a9effff),
            border: rgba_to_hsla(0x2a2a2aff),
            surface: rgba_to_hsla(0x1a1a1aff),
            muted: rgba_to_hsla(0x6b6b6bff),
        }
    }
}

/// Convert a packed `0xRRGGBBAA` hex literal into an [`Hsla`] colour.
fn rgba_to_hsla(packed: u32) -> Hsla {
    let r = ((packed >> 24) & 0xff) as f32 / 255.0;
    let g = ((packed >> 16) & 0xff) as f32 / 255.0;
    let b = ((packed >> 8) & 0xff) as f32 / 255.0;
    let a = (packed & 0xff) as f32 / 255.0;

    Rgba { r, g, b, a }.into()
}