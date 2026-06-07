//! Application-wide theme tokens.
//!
//! Provides a [`StorageWarsTheme`] struct that holds all colour values
//! used throughout the UI.  Currently only a dark variant is implemented.

use gpui::Hsla;

/// All colour tokens for the Storage Wars UI.
#[derive(Debug, Clone, Copy)]
pub struct StorageWarsTheme {
    /// Main window / panel background.
    pub background: Hsla,
    /// Slightly elevated surface (cards, sidebars).
    pub surface: Hsla,
    /// Primary interactive accent colour.
    pub accent: Hsla,
    /// Default body text.
    pub text: Hsla,
    /// Muted / secondary text.
    pub text_muted: Hsla,
    /// Subtle border / divider colour.
    pub border: Hsla,
    /// Destructive action colour.
    pub danger: Hsla,
    /// Success / positive colour.
    pub success: Hsla,
}

impl StorageWarsTheme {
    /// Returns the canonical dark theme.
    pub fn dark() -> Self {
        Self {
            // #0f1117 — near-black background
            background: Hsla {
                h: 228.0 / 360.0,
                s: 0.14,
                l: 0.08,
                a: 1.0,
            },
            // #1a1d27 — elevated surface
            surface: Hsla {
                h: 228.0 / 360.0,
                s: 0.20,
                l: 0.13,
                a: 1.0,
            },
            // #6c8ef5 — blue accent
            accent: Hsla {
                h: 228.0 / 360.0,
                s: 0.85,
                l: 0.69,
                a: 1.0,
            },
            // #e2e4f0 — primary text
            text: Hsla {
                h: 228.0 / 360.0,
                s: 0.30,
                l: 0.91,
                a: 1.0,
            },
            // #8b8fa8 — muted text
            text_muted: Hsla {
                h: 228.0 / 360.0,
                s: 0.12,
                l: 0.60,
                a: 1.0,
            },
            // #2a2d3e — subtle border
            border: Hsla {
                h: 228.0 / 360.0,
                s: 0.20,
                l: 0.21,
                a: 1.0,
            },
            // #f56c6c — danger red
            danger: Hsla {
                h: 0.0 / 360.0,
                s: 0.85,
                l: 0.69,
                a: 1.0,
            },
            // #67c23a — success green
            success: Hsla {
                h: 100.0 / 360.0,
                s: 0.56,
                l: 0.49,
                a: 1.0,
            },
        }
    }
}