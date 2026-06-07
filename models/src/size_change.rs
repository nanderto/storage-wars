//! Size change delta classification with associated display colors.

use serde::{Deserialize, Serialize};

/// Classifies the magnitude and direction of a size change between two scan
/// sessions. Each variant carries a hex color string suitable for UI rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeChange {
    /// Size increased significantly (> 20 %).
    LargeIncrease,
    /// Size increased moderately (5 – 20 %).
    SmallIncrease,
    /// Size is essentially unchanged (< 5 % in either direction).
    Unchanged,
    /// Size decreased moderately (5 – 20 %).
    SmallDecrease,
    /// Size decreased significantly (> 20 %).
    LargeDecrease,
    /// No previous size data is available for comparison.
    Unknown,
}

impl SizeChange {
    /// Returns the hex color string associated with this classification.
    ///
    /// Colors follow a traffic-light convention:
    /// - Red shades for increases (more disk usage).
    /// - Green shades for decreases (less disk usage).
    /// - Grey for unchanged or unknown.
    pub fn hex_color(&self) -> &'static str {
        match self {
            SizeChange::LargeIncrease => "#D32F2F",
            SizeChange::SmallIncrease => "#FF7043",
            SizeChange::Unchanged => "#9E9E9E",
            SizeChange::SmallDecrease => "#66BB6A",
            SizeChange::LargeDecrease => "#2E7D32",
            SizeChange::Unknown => "#BDBDBD",
        }
    }

    /// Classifies a size change given the `current` and `previous` sizes in
    /// bytes. Returns [`SizeChange::Unknown`] when `previous` is zero to
    /// avoid division by zero.
    pub fn classify(current: u64, previous: u64) -> Self {
        if previous == 0 {
            return SizeChange::Unknown;
        }

        let ratio = current as f64 / previous as f64;

        match ratio {
            r if r > 1.20 => SizeChange::LargeIncrease,
            r if r > 1.05 => SizeChange::SmallIncrease,
            r if r >= 0.95 => SizeChange::Unchanged,
            r if r >= 0.80 => SizeChange::SmallDecrease,
            _ => SizeChange::LargeDecrease,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_large_increase() {
        assert_eq!(SizeChange::classify(200, 100), SizeChange::LargeIncrease);
    }

    #[test]
    fn classify_small_increase() {
        assert_eq!(SizeChange::classify(110, 100), SizeChange::SmallIncrease);
    }

    #[test]
    fn classify_unchanged() {
        assert_eq!(SizeChange::classify(100, 100), SizeChange::Unchanged);
    }

    #[test]
    fn classify_small_decrease() {
        assert_eq!(SizeChange::classify(85, 100), SizeChange::SmallDecrease);
    }

    #[test]
    fn classify_large_decrease() {
        assert_eq!(SizeChange::classify(50, 100), SizeChange::LargeDecrease);
    }

    #[test]
    fn classify_unknown_when_previous_is_zero() {
        assert_eq!(SizeChange::classify(100, 0), SizeChange::Unknown);
    }

    #[test]
    fn hex_color_is_nonempty_for_all_variants() {
        let variants = [
            SizeChange::LargeIncrease,
            SizeChange::SmallIncrease,
            SizeChange::Unchanged,
            SizeChange::SmallDecrease,
            SizeChange::LargeDecrease,
            SizeChange::Unknown,
        ];
        for v in &variants {
            assert!(!v.hex_color().is_empty());
        }
    }
}