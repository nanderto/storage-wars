//! Application theme — colour palette and typography constants for the dark
//! theme used throughout Storage Wars.

use gpui::{hsla, Hsla};

/// The Storage Wars colour theme.
///
/// All colours are expressed as HSLA values so they can be passed directly to
/// GPUI style methods.
#[derive(Debug, Clone)]
pub struct StorageWarsTheme {
    // ── Surfaces ──────────────────────────────────────────────────────────────
    /// Primary window / panel background.
    pub background: Hsla,
    /// Slightly elevated surface (cards, sidebars).
    pub surface: Hsla,
    /// Highest elevation surface (modals, popovers).
    pub overlay: Hsla,

    // ── Borders ───────────────────────────────────────────────────────────────
    /// Subtle border between UI regions.
    pub border: Hsla,
    /// Focused / active border.
    pub border_focused: Hsla,

    // ── Text ──────────────────────────────────────────────────────────────────
    /// Primary text colour.
    pub foreground: Hsla,
    /// Secondary / muted text colour.
    pub foreground_muted: Hsla,
    /// Disabled text colour.
    pub foreground_disabled: Hsla,

    // ── Accent ────────────────────────────────────────────────────────────────
    /// Brand accent — used for interactive elements and highlights.
    pub accent: Hsla,
    /// Accent colour on hover.
    pub accent_hover: Hsla,
    /// Text rendered on top of the accent colour.
    pub accent_foreground: Hsla,

    // ── Status ────────────────────────────────────────────────────────────────
    pub success: Hsla,
    pub warning: Hsla,
    pub error: Hsla,

    // ── Title bar ─────────────────────────────────────────────────────────────
    /// Title bar background (slightly different from the main background).
    pub title_bar_background: Hsla,
    /// Title bar text / icon colour.
    pub title_bar_foreground: Hsla,
}

impl StorageWarsTheme {
    /// Returns the canonical dark theme for Storage Wars.
    pub fn dark() -> Self {
        Self {
            // Surfaces — deep charcoal palette
            background: hsla(220.0 / 360.0, 0.13, 0.11, 1.0),
            surface: hsla(220.0 / 360.0, 0.13, 0.15, 1.0),
            overlay: hsla(220.0 / 360.0, 0.13, 0.19, 1.0),

            // Borders
            border: hsla(220.0 / 360.0, 0.10, 0.25, 1.0),
            border_focused: hsla(220.0 / 360.0, 0.60, 0.55, 1.0),

            // Text
            foreground: hsla(220.0 / 360.0, 0.15, 0.90, 1.0),
            foreground_muted: hsla(220.0 / 360.0, 0.10, 0.60, 1.0),
            foreground_disabled: hsla(220.0 / 360.0, 0.05, 0.40, 1.0),

            // Accent — electric blue
            accent: hsla(217.0 / 360.0, 0.91, 0.60, 1.0),
            accent_hover: hsla(217.0 / 360.0, 0.91, 0.68, 1.0),
            accent_foreground: hsla(0.0, 0.0, 1.0, 1.0),

            // Status colours
            success: hsla(142.0 / 360.0, 0.71, 0.45, 1.0),
            warning: hsla(38.0 / 360.0, 0.92, 0.50, 1.0),
            error: hsla(0.0 / 360.0, 0.84, 0.60, 1.0),

            // Title bar — slightly darker than the main background
            title_bar_background: hsla(220.0 / 360.0, 0.13, 0.09, 1.0),
            title_bar_foreground: hsla(220.0 / 360.0, 0.15, 0.85, 1.0),
        }
    }
}