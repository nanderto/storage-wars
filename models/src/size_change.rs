//! Size change delta classification with associated display colors.

use serde::{Deserialize, Serialize};

/// Classifies the direction and magnitude of a size change between two scan sessions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeChangeTrend {
    /// The item is new (no previous size recorded).
    New,
    /// The item was deleted (present in a previous scan but not the current one).
    Deleted,
    /// The item grew significantly (more than the configured threshold).
    Increased,
    /// The item shrank significantly.
    Decreased,
    /// The item size is unchanged or changed within the noise threshold.
    Unchanged,
}

/// Represents the size delta between two scan sessions along with a trend classification
/// and a hex color code for UI rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeChange {
    /// The raw byte delta (positive = grew, negative = shrank).
    pub delta_bytes: i64,

    /// The percentage change relative to the previous size.
    /// `None` when there is no previous size (new items) or previous size was zero.
    pub delta_percent: Option<f64>,

    /// Trend classification for this change.
    pub trend: SizeChangeTrend,

    /// Hex color string (e.g. `"#FF4444"`) for rendering this change in the UI.
    pub color: String,
}

impl SizeChange {
    // ── Default color palette ────────────────────────────────────────────────

    /// Color used for new items.
    pub const COLOR_NEW: &'static str = "#4FC3F7";
    /// Color used for deleted items.
    pub const COLOR_DELETED: &'static str = "#EF5350";
    /// Color used for items that have grown.
    pub const COLOR_INCREASED: &'static str = "#FF7043";
    /// Color used for items that have shrunk.
    pub const COLOR_DECREASED: &'static str = "#66BB6A";
    /// Color used for unchanged items.
    pub const COLOR_UNCHANGED: &'static str = "#9E9E9E";

    /// Computes a `SizeChange` from a current and optional previous size.
    ///
    /// The `threshold_percent` parameter controls the minimum percentage change
    /// required to classify a change as [`SizeChangeTrend::Increased`] or
    /// [`SizeChangeTrend::Decreased`] rather than [`SizeChangeTrend::Unchanged`].
    pub fn compute(current: u64, prev: Option<u64>, threshold_percent: f64) -> Self {
        match prev {
            None => Self {
                delta_bytes: current as i64,
                delta_percent: None,
                trend: SizeChangeTrend::New,
                color: Self::COLOR_NEW.to_string(),
            },
            Some(0) if current > 0 => Self {
                delta_bytes: current as i64,
                delta_percent: None,
                trend: SizeChangeTrend::Increased,
                color: Self::COLOR_INCREASED.to_string(),
            },
            Some(prev_size) => {
                let delta = current as i64 - prev_size as i64;
                let pct = if prev_size == 0 {
                    None
                } else {
                    Some((delta as f64 / prev_size as f64) * 100.0)
                };

                let trend = match pct {
                    Some(p) if p > threshold_percent => SizeChangeTrend::Increased,
                    Some(p) if p < -threshold_percent => SizeChangeTrend::Decreased,
                    _ => SizeChangeTrend::Unchanged,
                };

                let color = match trend {
                    SizeChangeTrend::Increased => Self::COLOR_INCREASED,
                    SizeChangeTrend::Decreased => Self::COLOR_DECREASED,
                    _ => Self::COLOR_UNCHANGED,
                }
                .to_string();

                Self {
                    delta_bytes: delta,
                    delta_percent: pct,
                    trend,
                    color,
                }
            }
        }
    }

    /// Returns a `SizeChange` representing a deleted item.
    pub fn deleted(prev_size: u64) -> Self {
        Self {
            delta_bytes: -(prev_size as i64),
            delta_percent: Some(-100.0),
            trend: SizeChangeTrend::Deleted,
            color: Self::COLOR_DELETED.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_item() {
        let change = SizeChange::compute(1024, None, 5.0);
        assert_eq!(change.trend, SizeChangeTrend::New);
        assert_eq!(change.color, SizeChange::COLOR_NEW);
        assert!(change.delta_percent.is_none());
    }

    #[test]
    fn test_unchanged() {
        let change = SizeChange::compute(1024, Some(1024), 5.0);
        assert_eq!(change.trend, SizeChangeTrend::Unchanged);
        assert_eq!(change.delta_bytes, 0);
    }

    #[test]
    fn test_increased() {
        let change = SizeChange::compute(2000, Some(1000), 5.0);
        assert_eq!(change.trend, SizeChangeTrend::Increased);
        assert_eq!(change.color, SizeChange::COLOR_INCREASED);
        assert!((change.delta_percent.unwrap() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_decreased() {
        let change = SizeChange::compute(500, Some(1000), 5.0);
        assert_eq!(change.trend, SizeChangeTrend::Decreased);
        assert_eq!(change.color, SizeChange::COLOR_DECREASED);
        assert!((change.delta_percent.unwrap() - (-50.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_within_threshold_is_unchanged() {
        // 3% change with 5% threshold → unchanged
        let change = SizeChange::compute(1030, Some(1000), 5.0);
        assert_eq!(change.trend, SizeChangeTrend::Unchanged);
    }

    #[test]
    fn test_deleted() {
        let change = SizeChange::deleted(4096);
        assert_eq!(change.trend, SizeChangeTrend::Deleted);
        assert_eq!(change.delta_bytes, -4096);
        assert_eq!(change.delta_percent, Some(-100.0));
        assert_eq!(change.color, SizeChange::COLOR_DELETED);
    }
}