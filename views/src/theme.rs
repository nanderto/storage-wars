//! Design tokens — colours, spacing, typography used across all view components.

use gpui::{rgb, Rgba};

// ── Palette ──────────────────────────────────────────────────────────────────

pub const COLOR_BACKGROUND: Rgba = rgb(0x1E1E2E);
pub const COLOR_SURFACE: Rgba = rgb(0x27273A);
pub const COLOR_SURFACE_RAISED: Rgba = rgb(0x313145);
pub const COLOR_BORDER: Rgba = rgb(0x45455F);
pub const COLOR_TEXT_PRIMARY: Rgba = rgb(0xCDD6F4);
pub const COLOR_TEXT_SECONDARY: Rgba = rgb(0x9399B2);
pub const COLOR_TEXT_MUTED: Rgba = rgb(0x6C7086);
pub const COLOR_ACCENT: Rgba = rgb(0x89B4FA);
pub const COLOR_ACCENT_HOVER: Rgba = rgb(0xB4D0FF);
pub const COLOR_SELECTED: Rgba = rgb(0x313F5A);
pub const COLOR_SELECTED_BORDER: Rgba = rgb(0x89B4FA);

/// Progress bar / size-change colours.
pub const COLOR_SIZE_INCREASED: Rgba = rgb(0xF38BA8);
pub const COLOR_SIZE_DECREASED: Rgba = rgb(0xA6E3A1);
pub const COLOR_SIZE_UNCHANGED: Rgba = rgb(0x89B4FA);

pub const COLOR_BUTTON_BG: Rgba = rgb(0x313145);
pub const COLOR_BUTTON_BG_HOVER: Rgba = rgb(0x45455F);
pub const COLOR_BUTTON_DANGER: Rgba = rgb(0xF38BA8);
pub const COLOR_BUTTON_DANGER_HOVER: Rgba = rgb(0xFAA8B8);

pub const COLOR_TITLE_BAR_BG: Rgba = rgb(0x181825);
pub const COLOR_WINDOW_CONTROL_CLOSE: Rgba = rgb(0xF38BA8);
pub const COLOR_WINDOW_CONTROL_MINIMIZE: Rgba = rgb(0xF9E2AF);
pub const COLOR_WINDOW_CONTROL_MAXIMIZE: Rgba = rgb(0xA6E3A1);

// ── Spacing ───────────────────────────────────────────────────────────────────

pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 24.0;

/// Indentation per depth level in the tree view (px).
pub const TREE_INDENT_PX: f32 = 16.0;

/// Fixed width of the scan-history panel (px).
pub const SCAN_HISTORY_WIDTH: f32 = 280.0;

// ── Typography ────────────────────────────────────────────────────────────────

pub const FONT_SIZE_SM: f32 = 12.0;
pub const FONT_SIZE_MD: f32 = 14.0;
pub const FONT_SIZE_LG: f32 = 16.0;

// ── Dimensions ────────────────────────────────────────────────────────────────

pub const TITLE_BAR_HEIGHT: f32 = 32.0;
pub const DRIVE_SELECTOR_HEIGHT: f32 = 36.0;
pub const TREE_ROW_HEIGHT: f32 = 28.0;
pub const HISTORY_ROW_HEIGHT: f32 = 48.0;
pub const WINDOW_CONTROL_SIZE: f32 = 12.0;
pub const PROGRESS_BAR_HEIGHT: f32 = 6.0;