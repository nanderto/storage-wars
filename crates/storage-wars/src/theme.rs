//! Dark theme colour palette for Storage Wars.
//!
//! All colours are defined as [`gpui::Rgba`] constants so they can be used
//! directly in GPUI style methods (`.bg(theme::BACKGROUND)`, etc.).

use gpui::{rgb, Rgba};

// ── Backgrounds ───────────────────────────────────────────────────────────────

/// Primary application background — deep dark slate.
pub const BACKGROUND: Rgba = rgb(0x0f1117);

/// Secondary / surface background (panels, sidebars).
pub const SURFACE: Rgba = rgb(0x1a1d27);

/// Elevated surface (cards, modals, dropdowns).
pub const ELEVATED: Rgba = rgb(0x22263a);

// ── Borders ───────────────────────────────────────────────────────────────────

/// Default border / divider colour.
pub const BORDER: Rgba = rgb(0x2e3347);

/// Subtle border (used for inner separators).
pub const BORDER_SUBTLE: Rgba = rgb(0x1e2235);

// ── Text ──────────────────────────────────────────────────────────────────────

/// Primary text — near white.
pub const TEXT_PRIMARY: Rgba = rgb(0xe8eaf0);

/// Secondary / muted text.
pub const TEXT_SECONDARY: Rgba = rgb(0x8b91a8);

/// Disabled / placeholder text.
pub const TEXT_DISABLED: Rgba = rgb(0x4a5068);

// ── Accent (auction amber) ────────────────────────────────────────────────────

/// Brand accent — auction amber.
pub const ACCENT: Rgba = rgb(0xf5a623);

/// Accent hover state.
pub const ACCENT_HOVER: Rgba = rgb(0xf7b84b);

/// Accent pressed / active state.
pub const ACCENT_ACTIVE: Rgba = rgb(0xe09415);

/// Accent foreground text (used on accent-coloured backgrounds).
pub const ACCENT_FOREGROUND: Rgba = rgb(0x0f1117);

// ── Semantic colours ──────────────────────────────────────────────────────────

/// Success / positive indicator (green).
pub const SUCCESS: Rgba = rgb(0x4caf7d);

/// Warning indicator (amber — same hue as accent).
pub const WARNING: Rgba = rgb(0xf5a623);

/// Error / destructive indicator (red).
pub const ERROR: Rgba = rgb(0xef5350);

/// Informational indicator (blue).
pub const INFO: Rgba = rgb(0x42a5f5);