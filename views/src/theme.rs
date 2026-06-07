//! Design tokens — colours, spacing, and typography used across all views.

use gpui::{Hsla, Rgba, hsla, rgb};

// ---------------------------------------------------------------------------
// Palette
// ---------------------------------------------------------------------------

pub const BACKGROUND: Hsla = hsla(220.0 / 360.0, 0.13, 0.11, 1.0);
pub const SURFACE: Hsla = hsla(220.0 / 360.0, 0.12, 0.15, 1.0);
pub const SURFACE_RAISED: Hsla = hsla(220.0 / 360.0, 0.11, 0.18, 1.0);
pub const BORDER: Hsla = hsla(220.0 / 360.0, 0.10, 0.25, 1.0);

pub const TEXT_PRIMARY: Hsla = hsla(0.0, 0.0, 0.92, 1.0);
pub const TEXT_SECONDARY: Hsla = hsla(0.0, 0.0, 0.60, 1.0);
pub const TEXT_MUTED: Hsla = hsla(0.0, 0.0, 0.40, 1.0);

pub const ACCENT: Hsla = hsla(210.0 / 360.0, 0.80, 0.55, 1.0);
pub const ACCENT_HOVER: Hsla = hsla(210.0 / 360.0, 0.80, 0.65, 1.0);

// SizeChange colours
pub const COLOR_NEW: Hsla = hsla(142.0 / 360.0, 0.69, 0.45, 1.0);
pub const COLOR_DELETED: Hsla = hsla(0.0 / 360.0, 0.65, 0.50, 1.0);
pub const COLOR_GREW: Hsla = hsla(25.0 / 360.0, 0.90, 0.55, 1.0);
pub const COLOR_SHRANK: Hsla = hsla(195.0 / 360.0, 0.70, 0.50, 1.0);
pub const COLOR_UNCHANGED: Hsla = hsla(0.0, 0.0, 0.45, 1.0);

// Title bar
pub const TITLE_BAR_BG: Hsla = hsla(220.0 / 360.0, 0.14, 0.09, 1.0);
pub const TITLE_BAR_TEXT: Hsla = hsla(0.0, 0.0, 0.80, 1.0);

// Scan history panel
pub const HISTORY_PANEL_WIDTH: f32 = 280.0;
pub const HISTORY_BG: Hsla = hsla(220.0 / 360.0, 0.13, 0.12, 1.0);
pub const HISTORY_ITEM_HOVER: Hsla = hsla(220.0 / 360.0, 0.12, 0.20, 1.0);
pub const HISTORY_ITEM_SELECTED: Hsla = hsla(210.0 / 360.0, 0.50, 0.28, 1.0);

// Tree view
pub const TREE_INDENT_PX: f32 = 16.0;
pub const TREE_ROW_HEIGHT: f32 = 24.0;
pub const TREE_HEADER_HEIGHT: f32 = 28.0;
pub const TREE_ROW_HOVER: Hsla = hsla(220.0 / 360.0, 0.12, 0.20, 1.0);
pub const TREE_ROW_SELECTED: Hsla = hsla(210.0 / 360.0, 0.45, 0.25, 1.0);
pub const TREE_ROW_ALT: Hsla = hsla(220.0 / 360.0, 0.12, 0.13, 1.0);

// Spacing
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 24.0;

// Typography
pub const FONT_SIZE_SM: f32 = 11.0;
pub const FONT_SIZE_MD: f32 = 13.0;
pub const FONT_SIZE_LG: f32 = 15.0;

use crate::types::SizeChange;

/// Maps a [`SizeChange`] variant to its theme colour.
pub fn size_change_color(change: SizeChange) -> Hsla {
    match change {
        SizeChange::New => COLOR_NEW,
        SizeChange::Deleted => COLOR_DELETED,
        SizeChange::Grew => COLOR_GREW,
        SizeChange::Shrank => COLOR_SHRANK,
        SizeChange::Unchanged => COLOR_UNCHANGED,
    }
}