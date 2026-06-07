//! Size change delta classification with associated display colors.

use serde::{Deserialize, Serialize};

/// Classifies the direction and magnitude of a size change between scans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SizeChangeTrend {
    /// The item is new (no previous scan data).
    New,
    /// The item has grown significantly.
    LargeIncrease,
    /// The item has grown slightly.
    SmallIncrease,
    /// The item size is unchanged.
    Unchanged,
    /// The item has shrunk slightly.
    SmallDecrease,
    /// The item has shrunk significantly.
    LargeDecrease,
    /// The item no longer exists (deleted).
    Deleted,
}

/// Represents the size change of a filesystem node between two scans,
/// including a trend classification and a hex color for UI display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeChange {
    /// The absolute size delta in bytes (positive = grew, negative = shrank).
    pub delta_bytes: i64,

    /// The relative change as a percentage. `None` if there is no previous size.
    pub delta_percent: Option<f64>,

    /// The classified trend of the size change.
    pub trend: SizeChangeTrend,

    /// A hex color string (e.g. `"#FF5733"`) for rendering this change in the UI.
    pub hex_color: String,
}

impl SizeChange {
    /// Threshold in percent above which a change is considered "large".
    const LARGE_THRESHOLD_PERCENT: f64 = 20.0;

    /// Computes a `SizeChange` from a current size and an optional previous size.
    pub fn compute(current: u64, previous: Option<u64>) -> Self {
        match previous {
            None => Self {
                delta_bytes: current as i64,
                delta_percent: None,
                trend: SizeChangeTrend::New,
                hex_color: Self::color_for(SizeChangeTrend::New).to_string(),
            },
            Some(prev) => {
                let delta = current as i64 - prev as i64;
                let percent = if prev == 0 {
                    None
                } else {
                    Some((delta as f64 / prev as f64) * 100.0)
                };

                let trend = Self::classify(delta, percent);
                Self {
                    delta_bytes: delta,
                    delta_percent: percent,
                    trend,
                    hex_color: Self::color_for(trend).to_string(),
                }
            }
        }
    }

    /// Classifies the trend based on the delta and optional percentage.
    fn classify(delta: i64, percent: Option<f64>) -> SizeChangeTrend {
        if delta == 0 {
            return SizeChangeTrend::Unchanged;
        }

        match percent {
            Some(p) if p > Self::LARGE_THRESHOLD_PERCENT => SizeChangeTrend::LargeIncrease,
            Some(p) if p > 0.0 => SizeChangeTrend::SmallIncrease,
            Some(p) if p < -Self::LARGE_THRESHOLD_PERCENT => SizeChangeTrend::LargeDecrease,
            Some(p) if p < 0.0 => SizeChangeTrend::SmallDecrease,
            None if delta > 0 => SizeChangeTrend::LargeIncrease,
            None if delta < 0 => SizeChangeTrend::LargeDecrease,
            _ => SizeChangeTrend::Unchanged,
        }
    }

    /// Returns the canonical hex color for a given trend.
    pub fn color_for(trend: SizeChangeTrend) -> &'static str {
        match trend {
            SizeChangeTrend::New => "#3498DB",          // Blue
            SizeChangeTrend::LargeIncrease => "#E74C3C", // Red
            SizeChangeTrend::SmallIncrease => "#E67E22", // Orange
            SizeChangeTrend::Unchanged => "#95A5A6",     // Gray
            SizeChangeTrend::SmallDecrease => "#2ECC71", // Light green
            SizeChangeTrend::LargeDecrease => "#27AE60", // Dark green
            SizeChangeTrend::Deleted => "#8E44AD",       // Purple
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_item() {
        let change = SizeChange::compute(1024, None);
        assert_eq!(change.trend, SizeChangeTrend::New);
        assert_eq!(change.delta_bytes, 1024);
        assert!(change.delta_percent.is_none());
        assert_eq!(change.hex_color, "#3498DB");
    }

    #[test]
    fn test_unchanged() {
        let change = SizeChange::compute(1024, Some(1024));
        assert_eq!(change.trend, SizeChangeTrend::Unchanged);
        assert_eq!(change.delta_bytes, 0);
    }

    #[test]
    fn test_large_increase() {
        let change = SizeChange::compute(2000, Some(1000));
        assert_eq!(change.trend, SizeChangeTrend::LargeIncrease);
        assert!(change.delta_bytes > 0);
    }

    #[test]
    fn test_small_increase() {
        let change = SizeChange::compute(1050, Some(1000));
        assert_eq!(change.trend, SizeChangeTrend::SmallIncrease);
    }

    #[test]
    fn test_large_decrease() {
        let change = SizeChange::compute(500, Some(1000));
        assert_eq!(change.trend, SizeChangeTrend::LargeDecrease);
        assert!(change.delta_bytes < 0);
    }

    #[test]
    fn test_small_decrease() {
        let change = SizeChange::compute(950, Some(1000));
        assert_eq!(change.trend, SizeChangeTrend::SmallDecrease);
    }

    #[test]
    fn test_color_for_all_trends() {
        let trends = [
            SizeChangeTrend::New,
            SizeChangeTrend::LargeIncrease,
            SizeChangeTrend::SmallIncrease,
            SizeChangeTrend::Unchanged,
            SizeChangeTrend::SmallDecrease,
            SizeChangeTrend::LargeDecrease,
            SizeChangeTrend::Deleted,
        ];
        for trend in trends {
            let color = SizeChange::color_for(trend);
            assert!(color.starts_with('#'), "color for {trend:?} must start with '#'");
            assert_eq!(color.len(), 7, "color for {trend:?} must be 7 chars");
        }
    }

    #[test]
    fn test_serialization_roundtrip() {
        let change = SizeChange::compute(2048, Some(1024));
        let json = serde_json::to_string(&change).expect("serialization failed");
        let restored: SizeChange = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(change, restored);
    }
}