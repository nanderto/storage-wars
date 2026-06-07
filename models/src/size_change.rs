//! Size-change delta classification with associated display colors.

use serde::{Deserialize, Serialize};

/// Classifies the magnitude and direction of a size change between two scans.
///
/// Each variant carries a hex color string suitable for use in the UI to give
/// users an at-a-glance indication of how significantly a node's size has changed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeChangeTrend {
    /// The node is new (did not exist in the previous scan).
    New,
    /// The node was deleted (does not exist in the current scan).
    Deleted,
    /// The node grew significantly (> 20 % increase).
    LargeIncrease,
    /// The node grew moderately (5 %–20 % increase).
    ModerateIncrease,
    /// The node grew slightly (< 5 % increase).
    SlightIncrease,
    /// The node size is unchanged.
    Unchanged,
    /// The node shrank slightly (< 5 % decrease).
    SlightDecrease,
    /// The node shrank moderately (5 %–20 % decrease).
    ModerateDecrease,
    /// The node shrank significantly (> 20 % decrease).
    LargeDecrease,
}

impl SizeChangeTrend {
    /// Returns the hex color code associated with this trend for UI rendering.
    pub fn hex_color(&self) -> &'static str {
        match self {
            SizeChangeTrend::New => "#00C853",
            SizeChangeTrend::Deleted => "#B71C1C",
            SizeChangeTrend::LargeIncrease => "#D32F2F",
            SizeChangeTrend::ModerateIncrease => "#F57C00",
            SizeChangeTrend::SlightIncrease => "#FBC02D",
            SizeChangeTrend::Unchanged => "#9E9E9E",
            SizeChangeTrend::SlightDecrease => "#81D4FA",
            SizeChangeTrend::ModerateDecrease => "#29B6F6",
            SizeChangeTrend::LargeDecrease => "#0288D1",
        }
    }

    /// Returns a short human-readable label for this trend.
    pub fn label(&self) -> &'static str {
        match self {
            SizeChangeTrend::New => "New",
            SizeChangeTrend::Deleted => "Deleted",
            SizeChangeTrend::LargeIncrease => "Large Increase",
            SizeChangeTrend::ModerateIncrease => "Moderate Increase",
            SizeChangeTrend::SlightIncrease => "Slight Increase",
            SizeChangeTrend::Unchanged => "Unchanged",
            SizeChangeTrend::SlightDecrease => "Slight Decrease",
            SizeChangeTrend::ModerateDecrease => "Moderate Decrease",
            SizeChangeTrend::LargeDecrease => "Large Decrease",
        }
    }
}

/// Represents the size change of a node between two scan sessions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeChange {
    /// The absolute size delta in bytes (positive = grew, negative = shrank).
    pub delta_bytes: i64,

    /// The percentage change relative to the previous size.
    /// `None` if the node is new or was deleted.
    pub delta_percent: Option<f64>,

    /// The classified trend for this change.
    pub trend: SizeChangeTrend,
}

impl SizeChange {
    /// Computes a [`SizeChange`] given the current and previous sizes.
    ///
    /// Pass `None` for `prev_size` to indicate a new node.
    /// Pass `None` for `current_size` to indicate a deleted node.
    pub fn compute(current_size: Option<u64>, prev_size: Option<u64>) -> Self {
        match (current_size, prev_size) {
            (Some(_), None) => Self {
                delta_bytes: current_size.unwrap() as i64,
                delta_percent: None,
                trend: SizeChangeTrend::New,
            },
            (None, Some(prev)) => Self {
                delta_bytes: -(prev as i64),
                delta_percent: None,
                trend: SizeChangeTrend::Deleted,
            },
            (None, None) => Self {
                delta_bytes: 0,
                delta_percent: Some(0.0),
                trend: SizeChangeTrend::Unchanged,
            },
            (Some(current), Some(prev)) => {
                let delta = current as i64 - prev as i64;
                let percent = if prev == 0 {
                    if current == 0 {
                        0.0
                    } else {
                        100.0
                    }
                } else {
                    (delta as f64 / prev as f64) * 100.0
                };

                let trend = classify_percent(percent);

                Self {
                    delta_bytes: delta,
                    delta_percent: Some(percent),
                    trend,
                }
            }
        }
    }

    /// Returns the hex color for the associated trend.
    pub fn hex_color(&self) -> &'static str {
        self.trend.hex_color()
    }
}

/// Classifies a percentage change into a [`SizeChangeTrend`].
fn classify_percent(percent: f64) -> SizeChangeTrend {
    if percent > 20.0 {
        SizeChangeTrend::LargeIncrease
    } else if percent > 5.0 {
        SizeChangeTrend::ModerateIncrease
    } else if percent > 0.0 {
        SizeChangeTrend::SlightIncrease
    } else if percent == 0.0 {
        SizeChangeTrend::Unchanged
    } else if percent >= -5.0 {
        SizeChangeTrend::SlightDecrease
    } else if percent >= -20.0 {
        SizeChangeTrend::ModerateDecrease
    } else {
        SizeChangeTrend::LargeDecrease
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_classified_correctly() {
        let change = SizeChange::compute(Some(1024), None);
        assert_eq!(change.trend, SizeChangeTrend::New);
        assert_eq!(change.delta_bytes, 1024);
        assert!(change.delta_percent.is_none());
    }

    #[test]
    fn deleted_node_classified_correctly() {
        let change = SizeChange::compute(None, Some(2048));
        assert_eq!(change.trend, SizeChangeTrend::Deleted);
        assert_eq!(change.delta_bytes, -2048);
    }

    #[test]
    fn unchanged_node_classified_correctly() {
        let change = SizeChange::compute(Some(500), Some(500));
        assert_eq!(change.trend, SizeChangeTrend::Unchanged);
        assert_eq!(change.delta_bytes, 0);
    }

    #[test]
    fn large_increase_classified_correctly() {
        let change = SizeChange::compute(Some(1000), Some(100));
        assert_eq!(change.trend, SizeChangeTrend::LargeIncrease);
    }

    #[test]
    fn large_decrease_classified_correctly() {
        let change = SizeChange::compute(Some(100), Some(1000));
        assert_eq!(change.trend, SizeChangeTrend::LargeDecrease);
    }

    #[test]
    fn hex_color_is_non_empty() {
        for trend in [
            SizeChangeTrend::New,
            SizeChangeTrend::Deleted,
            SizeChangeTrend::LargeIncrease,
            SizeChangeTrend::ModerateIncrease,
            SizeChangeTrend::SlightIncrease,
            SizeChangeTrend::Unchanged,
            SizeChangeTrend::SlightDecrease,
            SizeChangeTrend::ModerateDecrease,
            SizeChangeTrend::LargeDecrease,
        ] {
            assert!(trend.hex_color().starts_with('#'));
        }
    }

    #[test]
    fn serialization_round_trip() {
        let change = SizeChange::compute(Some(2000), Some(1000));
        let json = serde_json::to_string(&change).unwrap();
        let restored: SizeChange = serde_json::from_str(&json).unwrap();
        assert_eq!(change, restored);
    }
}