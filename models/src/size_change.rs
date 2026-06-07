//! Size change delta classification with associated display colors.

use serde::{Deserialize, Serialize};

/// Classifies the magnitude and direction of a size change between two scans,
/// and provides a hex color code suitable for UI rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeChange {
    /// The entry is new (did not exist in the previous scan).
    New,
    /// The entry has grown significantly (> 20% increase).
    LargeIncrease,
    /// The entry has grown slightly (1–20% increase).
    SmallIncrease,
    /// The entry size is unchanged.
    Unchanged,
    /// The entry has shrunk slightly (1–20% decrease).
    SmallDecrease,
    /// The entry has shrunk significantly (> 20% decrease).
    LargeDecrease,
    /// The entry no longer exists (was deleted since the previous scan).
    Deleted,
}

impl SizeChange {
    /// Returns the hex color string associated with this change classification.
    ///
    /// Colors are intended for use in UI components to give immediate visual
    /// feedback about the nature of the size change.
    pub fn hex_color(&self) -> &'static str {
        match self {
            SizeChange::New => "#4CAF50",          // green
            SizeChange::LargeIncrease => "#F44336", // red
            SizeChange::SmallIncrease => "#FF9800",  // orange
            SizeChange::Unchanged => "#9E9E9E",      // grey
            SizeChange::SmallDecrease => "#03A9F4",  // light blue
            SizeChange::LargeDecrease => "#2196F3",  // blue
            SizeChange::Deleted => "#607D8B",        // blue-grey
        }
    }

    /// Classifies a size change given the previous and current sizes.
    ///
    /// # Arguments
    /// * `prev` – size in bytes from the previous scan (`None` if the entry is new).
    /// * `current` – current size in bytes (`None` if the entry was deleted).
    pub fn classify(prev: Option<u64>, current: Option<u64>) -> Self {
        match (prev, current) {
            (None, Some(_)) => SizeChange::New,
            (Some(_), None) => SizeChange::Deleted,
            (None, None) => SizeChange::Unchanged,
            (Some(p), Some(c)) => {
                if p == c {
                    return SizeChange::Unchanged;
                }
                // Avoid division by zero for zero-sized previous entries.
                if p == 0 {
                    return if c > 0 {
                        SizeChange::LargeIncrease
                    } else {
                        SizeChange::Unchanged
                    };
                }
                let ratio = c as f64 / p as f64;
                if ratio > 1.20 {
                    SizeChange::LargeIncrease
                } else if ratio > 1.0 {
                    SizeChange::SmallIncrease
                } else if ratio < 0.80 {
                    SizeChange::LargeDecrease
                } else {
                    SizeChange::SmallDecrease
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_entry_classified_correctly() {
        assert_eq!(SizeChange::classify(None, Some(1024)), SizeChange::New);
    }

    #[test]
    fn deleted_entry_classified_correctly() {
        assert_eq!(SizeChange::classify(Some(1024), None), SizeChange::Deleted);
    }

    #[test]
    fn unchanged_entry_classified_correctly() {
        assert_eq!(SizeChange::classify(Some(512), Some(512)), SizeChange::Unchanged);
    }

    #[test]
    fn large_increase_classified_correctly() {
        assert_eq!(SizeChange::classify(Some(100), Some(200)), SizeChange::LargeIncrease);
    }

    #[test]
    fn small_increase_classified_correctly() {
        assert_eq!(SizeChange::classify(Some(100), Some(110)), SizeChange::SmallIncrease);
    }

    #[test]
    fn large_decrease_classified_correctly() {
        assert_eq!(SizeChange::classify(Some(200), Some(100)), SizeChange::LargeDecrease);
    }

    #[test]
    fn small_decrease_classified_correctly() {
        assert_eq!(SizeChange::classify(Some(200), Some(190)), SizeChange::SmallDecrease);
    }

    #[test]
    fn hex_colors_are_valid_hex_strings() {
        let variants = [
            SizeChange::New,
            SizeChange::LargeIncrease,
            SizeChange::SmallIncrease,
            SizeChange::Unchanged,
            SizeChange::SmallDecrease,
            SizeChange::LargeDecrease,
            SizeChange::Deleted,
        ];
        for variant in &variants {
            let color = variant.hex_color();
            assert!(color.starts_with('#'), "color {color} should start with #");
            assert_eq!(color.len(), 7, "color {color} should be 7 chars");
        }
    }
}