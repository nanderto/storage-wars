//! Shared theme constants used across view components.

use gpui::{Hsla, hsla};

/// Background color for the main application surface.
pub const COLOR_BACKGROUND: Hsla = hsla(0.0, 0.0, 0.12, 1.0);

/// Surface color for panels and cards.
pub const COLOR_SURFACE: Hsla = hsla(0.0, 0.0, 0.16, 1.0);

/// Border color for interactive elements.
pub const COLOR_BORDER: Hsla = hsla(0.0, 0.0, 0.25, 1.0);

/// Primary text color.
pub const COLOR_TEXT_PRIMARY: Hsla = hsla(0.0, 0.0, 0.92, 1.0);

/// Secondary / muted text color.
pub const COLOR_TEXT_SECONDARY: Hsla = hsla(0.0, 0.0, 0.60, 1.0);

/// Accent / highlight color.
pub const COLOR_ACCENT: Hsla = hsla(0.60, 0.80, 0.55, 1.0);

/// Color used when a size has grown compared to the previous scan.
pub const COLOR_SIZE_GREW: Hsla = hsla(0.0, 0.75, 0.55, 1.0);

/// Color used when a size has shrunk compared to the previous scan.
pub const COLOR_SIZE_SHRANK: Hsla = hsla(0.33, 0.65, 0.45, 1.0);

/// Color used when a size is unchanged compared to the previous scan.
pub const COLOR_SIZE_UNCHANGED: Hsla = hsla(0.0, 0.0, 0.45, 1.0);

/// Indentation step in pixels per depth level for the tree view.
pub const TREE_INDENT_PX: f32 = 16.0;

/// Fixed width of the [`ScanHistory`] panel in pixels.
pub const SCAN_HISTORY_WIDTH_PX: f32 = 280.0;

/// Represents the direction of size change between two scans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeChange {
    Grew,
    Shrank,
    Unchanged,
}

impl SizeChange {
    /// Returns the theme color associated with this change direction.
    pub fn color(self) -> Hsla {
        match self {
            SizeChange::Grew => COLOR_SIZE_GREW,
            SizeChange::Shrank => COLOR_SIZE_SHRANK,
            SizeChange::Unchanged => COLOR_SIZE_UNCHANGED,
        }
    }
}