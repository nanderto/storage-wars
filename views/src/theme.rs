//! Colour palette and spacing constants shared across all view components.

use gpui::{hsla, rgb, Hsla, Rgba};

// ---------------------------------------------------------------------------
// Spacing
// ---------------------------------------------------------------------------

/// Indentation step per depth level in the tree view (px).
pub const TREE_INDENT_PX: f32 = 16.0;

/// Width of the scan-history panel (px).
pub const HISTORY_PANEL_WIDTH_PX: f32 = 280.0;

/// Height of a single row in the tree view (px).
pub const TREE_ROW_HEIGHT_PX: f32 = 24.0;

/// Height of the custom title bar (px).
pub const TITLE_BAR_HEIGHT_PX: f32 = 36.0;

// ---------------------------------------------------------------------------
// Colours
// ---------------------------------------------------------------------------

/// Background colour for the main application window.
pub fn bg_primary() -> Hsla {
    hsla(220.0 / 360.0, 0.13, 0.11, 1.0)
}

/// Secondary / panel background.
pub fn bg_secondary() -> Hsla {
    hsla(220.0 / 360.0, 0.12, 0.15, 1.0)
}

/// Hover highlight.
pub fn bg_hover() -> Hsla {
    hsla(220.0 / 360.0, 0.15, 0.20, 1.0)
}

/// Selected-row highlight.
pub fn bg_selected() -> Hsla {
    hsla(213.0 / 360.0, 0.60, 0.35, 1.0)
}

/// Primary text colour.
pub fn text_primary() -> Hsla {
    hsla(0.0, 0.0, 0.92, 1.0)
}

/// Secondary / muted text colour.
pub fn text_secondary() -> Hsla {
    hsla(0.0, 0.0, 0.60, 1.0)
}

/// Accent / interactive colour.
pub fn accent() -> Hsla {
    hsla(213.0 / 360.0, 0.90, 0.60, 1.0)
}

/// Progress-bar fill for a node that has grown.
pub fn size_grown() -> Hsla {
    hsla(0.0 / 360.0, 0.75, 0.55, 1.0)
}

/// Progress-bar fill for a node that has shrunk.
pub fn size_shrunk() -> Hsla {
    hsla(142.0 / 360.0, 0.60, 0.45, 1.0)
}

/// Progress-bar fill for an unchanged node.
pub fn size_unchanged() -> Hsla {
    hsla(213.0 / 360.0, 0.50, 0.50, 1.0)
}

/// Border / separator colour.
pub fn border() -> Hsla {
    hsla(220.0 / 360.0, 0.10, 0.25, 1.0)
}

/// Title-bar background.
pub fn title_bar_bg() -> Hsla {
    hsla(220.0 / 360.0, 0.14, 0.09, 1.0)
}

/// Destructive action colour (delete button).
pub fn destructive() -> Hsla {
    hsla(0.0 / 360.0, 0.70, 0.50, 1.0)
}