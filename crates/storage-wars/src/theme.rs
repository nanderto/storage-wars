use gpui::{Hsla, Rgba};

/// Dark theme colour palette for Storage Wars.
#[derive(Debug, Clone)]
pub struct StorageWarsTheme {
    /// Main window background.
    pub background: Hsla,
    /// Primary text / icon colour.
    pub foreground: Hsla,
    /// Secondary / muted text.
    pub muted: Hsla,
    /// Accent / highlight colour.
    pub accent: Hsla,
    /// Surface colour for cards and panels.
    pub surface: Hsla,
    /// Border colour.
    pub border: Hsla,
    /// Title-bar background.
    pub title_bar_background: Hsla,
}

impl StorageWarsTheme {
    /// Construct the canonical dark theme.
    pub fn dark() -> Self {
        Self {
            background: rgba(0x0d0d0dff),
            foreground: rgba(0xe8e8e8ff),
            muted: rgba(0x888888ff),
            accent: rgba(0x4a9effff),
            surface: rgba(0x1a1a1aff),
            border: rgba(0x2a2a2aff),
            title_bar_background: rgba(0x111111ff),
        }
    }
}

/// Convert a packed `0xRRGGBBAA` value into an [`Hsla`].
fn rgba(packed: u32) -> Hsla {
    let r = ((packed >> 24) & 0xff) as f32 / 255.0;
    let g = ((packed >> 16) & 0xff) as f32 / 255.0;
    let b = ((packed >> 8) & 0xff) as f32 / 255.0;
    let a = (packed & 0xff) as f32 / 255.0;
    Rgba { r, g, b, a }.into()
}