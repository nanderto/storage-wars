//! Color theme and styling constants for the views component.

use gpui::{Hsla, Rgba, hsla, rgb};

/// Size change classification used for progress bar coloring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeChange {
    /// Item is new (did not exist in previous scan).
    New,
    /// Item grew significantly.
    Grown,
    /// Item is roughly the same size.
    Unchanged,
    /// Item shrank significantly.
    Shrunk,
    /// Item was deleted (does not exist in current scan).
    Deleted,
}

impl SizeChange {
    /// Returns the HSLA color associated with this size change variant.
    pub fn color(self) -> Hsla {
        match self {
            SizeChange::New => hsla(0.33, 0.65, 0.45, 1.0),      // green
            SizeChange::Grown => hsla(0.08, 0.85, 0.55, 1.0),    // orange-red
            SizeChange::Unchanged => hsla(0.60, 0.20, 0.55, 1.0), // muted blue
            SizeChange::Shrunk => hsla(0.55, 0.55, 0.50, 1.0),   // teal
            SizeChange::Deleted => hsla(0.0, 0.70, 0.45, 1.0),   // red
        }
    }

    /// Classifies a size change given old and new byte counts.
    ///
    /// # Arguments
    /// * `old` – previous scan size in bytes (`None` if item is new).
    /// * `new` – current scan size in bytes (`None` if item was deleted).
    pub fn classify(old: Option<u64>, new: Option<u64>) -> Self {
        match (old, new) {
            (None, Some(_)) => SizeChange::New,
            (Some(_), None) => SizeChange::Deleted,
            (Some(o), Some(n)) => {
                if o == 0 && n == 0 {
                    return SizeChange::Unchanged;
                }
                let ratio = if o == 0 {
                    f64::INFINITY
                } else {
                    n as f64 / o as f64
                };
                if ratio > 1.10 {
                    SizeChange::Grown
                } else if ratio < 0.90 {
                    SizeChange::Shrunk
                } else {
                    SizeChange::Unchanged
                }
            }
            (None, None) => SizeChange::Unchanged,
        }
    }
}

/// Application-wide color palette.
pub struct Palette;

impl Palette {
    pub fn background() -> Hsla {
        hsla(0.0, 0.0, 0.12, 1.0)
    }

    pub fn surface() -> Hsla {
        hsla(0.0, 0.0, 0.16, 1.0)
    }

    pub fn surface_elevated() -> Hsla {
        hsla(0.0, 0.0, 0.20, 1.0)
    }

    pub fn border() -> Hsla {
        hsla(0.0, 0.0, 0.28, 1.0)
    }

    pub fn text_primary() -> Hsla {
        hsla(0.0, 0.0, 0.92, 1.0)
    }

    pub fn text_secondary() -> Hsla {
        hsla(0.0, 0.0, 0.65, 1.0)
    }

    pub fn text_muted() -> Hsla {
        hsla(0.0, 0.0, 0.45, 1.0)
    }

    pub fn accent() -> Hsla {
        hsla(0.60, 0.70, 0.55, 1.0)
    }

    pub fn accent_hover() -> Hsla {
        hsla(0.60, 0.70, 0.65, 1.0)
    }

    pub fn selection() -> Hsla {
        hsla(0.60, 0.50, 0.35, 0.50)
    }

    pub fn progress_track() -> Hsla {
        hsla(0.0, 0.0, 0.25, 1.0)
    }

    pub fn title_bar() -> Hsla {
        hsla(0.0, 0.0, 0.10, 1.0)
    }
}

/// Indentation width per depth level in the tree view (pixels).
pub const TREE_INDENT_PX: f32 = 16.0;

/// Width of the scan history panel (pixels).
pub const SCAN_HISTORY_WIDTH_PX: f32 = 280.0;

/// Height of a single row in the tree view (pixels).
pub const TREE_ROW_HEIGHT_PX: f32 = 24.0;

/// Height of the custom title bar (pixels).
pub const TITLE_BAR_HEIGHT_PX: f32 = 36.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_new_item() {
        assert_eq!(SizeChange::classify(None, Some(1024)), SizeChange::New);
    }

    #[test]
    fn classify_deleted_item() {
        assert_eq!(SizeChange::classify(Some(1024), None), SizeChange::Deleted);
    }

    #[test]
    fn classify_grown_item() {
        assert_eq!(
            SizeChange::classify(Some(1000), Some(1200)),
            SizeChange::Grown
        );
    }

    #[test]
    fn classify_shrunk_item() {
        assert_eq!(
            SizeChange::classify(Some(1000), Some(800)),
            SizeChange::Shrunk
        );
    }

    #[test]
    fn classify_unchanged_item() {
        assert_eq!(
            SizeChange::classify(Some(1000), Some(1000)),
            SizeChange::Unchanged
        );
    }
}