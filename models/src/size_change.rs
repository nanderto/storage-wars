//! Size change delta classification with associated display colors.

use serde::{Deserialize, Serialize};

/// Classifies the magnitude and direction of a size change between two scans,
/// and provides a hex color string suitable for UI display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SizeChange {
    /// The item is new (no previous size recorded).
    New,
    /// The item was deleted (current size is zero, had a previous size).
    Deleted,
    /// The item grew significantly (> 20% increase).
    LargeIncrease,
    /// The item grew moderately (5–20% increase).
    ModerateIncrease,
    /// The item grew slightly (0–5% increase).
    SlightIncrease,
    /// The size is unchanged.
    Unchanged,
    /// The item shrank slightly (0–5% decrease).
    SlightDecrease,
    /// The item shrank moderately (5–20% decrease).
    ModerateDecrease,
    /// The item shrank significantly (> 20% decrease).
    LargeDecrease,
}

impl SizeChange {
    /// Returns the hex color string associated with this change classification.
    ///
    /// Colors follow a red (growth) → neutral (unchanged) → green (shrinkage)
    /// convention.
    pub fn hex_color(&self) -> &'static str {
        match self {
            SizeChange::New => "#A855F7",             // purple
            SizeChange::Deleted => "#6B7280",         // gray
            SizeChange::LargeIncrease => "#DC2626",   // red-600
            SizeChange::ModerateIncrease => "#F97316", // orange-500
            SizeChange::SlightIncrease => "#FCD34D",  // amber-300
            SizeChange::Unchanged => "#9CA3AF",       // gray-400
            SizeChange::SlightDecrease => "#86EFAC",  // green-300
            SizeChange::ModerateDecrease => "#22C55E", // green-500
            SizeChange::LargeDecrease => "#15803D",   // green-700
        }
    }

    /// Classifies a size change given the current and previous sizes.
    ///
    /// # Arguments
    /// * `current` – the current size in bytes.
    /// * `previous` – the previous size in bytes, or `None` if the item is new.
    pub fn classify(current: u64, previous: Option<u64>) -> Self {
        let Some(prev) = previous else {
            return SizeChange::New;
        };

        if current == 0 && prev > 0 {
            return SizeChange::Deleted;
        }

        if prev == 0 {
            // Avoid division by zero; treat any size as a large increase.
            return if current > 0 {
                SizeChange::LargeIncrease
            } else {
                SizeChange::Unchanged
            };
        }

        let ratio = current as f64 / prev as f64;

        match ratio {
            r if r > 1.20 => SizeChange::LargeIncrease,
            r if r > 1.05 => SizeChange::ModerateIncrease,
            r if r > 1.00 => SizeChange::SlightIncrease,
            r if (r - 1.00_f64).abs() < f64::EPSILON => SizeChange::Unchanged,
            r if r >= 0.95 => SizeChange::SlightDecrease,
            r if r >= 0.80 => SizeChange::ModerateDecrease,
            _ => SizeChange::LargeDecrease,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_when_no_previous() {
        assert_eq!(SizeChange::classify(500, None), SizeChange::New);
    }

    #[test]
    fn deleted_when_current_zero() {
        assert_eq!(SizeChange::classify(0, Some(100)), SizeChange::Deleted);
    }

    #[test]
    fn large_increase_over_20_percent() {
        assert_eq!(SizeChange::classify(130, Some(100)), SizeChange::LargeIncrease);
    }

    #[test]
    fn moderate_increase_between_5_and_20_percent() {
        assert_eq!(SizeChange::classify(110, Some(100)), SizeChange::ModerateIncrease);
    }

    #[test]
    fn unchanged_same_size() {
        assert_eq!(SizeChange::classify(100, Some(100)), SizeChange::Unchanged);
    }

    #[test]
    fn large_decrease_over_20_percent() {
        assert_eq!(SizeChange::classify(70, Some(100)), SizeChange::LargeDecrease);
    }

    #[test]
    fn hex_color_returns_nonempty_string() {
        for variant in [
            SizeChange::New,
            SizeChange::Deleted,
            SizeChange::LargeIncrease,
            SizeChange::ModerateIncrease,
            SizeChange::SlightIncrease,
            SizeChange::Unchanged,
            SizeChange::SlightDecrease,
            SizeChange::ModerateDecrease,
            SizeChange::LargeDecrease,
        ] {
            let color = variant.hex_color();
            assert!(color.starts_with('#'), "Expected hex color, got: {color}");
        }
    }
}