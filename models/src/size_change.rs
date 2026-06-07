//! Size change delta classification with associated display colors.

use serde::{Deserialize, Serialize};

/// Classifies the direction and magnitude of a size change between two scans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SizeChangeTrend {
    /// The item is new (no previous size recorded).
    New,
    /// The item was deleted (no current size, only previous).
    Deleted,
    /// The item grew significantly (above a configured threshold).
    IncreasedLarge,
    /// The item grew slightly.
    IncreasedSmall,
    /// The item shrank slightly.
    DecreasedSmall,
    /// The item shrank significantly.
    DecreasedLarge,
    /// No meaningful change in size.
    Unchanged,
}

/// Represents a classified size change between two scan sessions, including
/// the raw delta and a hex color string for UI rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SizeChange {
    /// The trend / classification of the size change.
    pub trend: SizeChangeTrend,

    /// The raw size delta in bytes (positive = grew, negative = shrank).
    pub delta_bytes: i64,

    /// A CSS-style hex color string (e.g. `"#FF4444"`) for UI rendering.
    pub hex_color: String,
}

impl SizeChangeTrend {
    /// Returns the canonical hex color associated with this trend.
    pub fn hex_color(self) -> &'static str {
        match self {
            SizeChangeTrend::New => "#4CAF50",
            SizeChangeTrend::Deleted => "#9E9E9E",
            SizeChangeTrend::IncreasedLarge => "#F44336",
            SizeChangeTrend::IncreasedSmall => "#FF9800",
            SizeChangeTrend::DecreasedSmall => "#8BC34A",
            SizeChangeTrend::DecreasedLarge => "#2196F3",
            SizeChangeTrend::Unchanged => "#BDBDBD",
        }
    }
}

impl SizeChange {
    /// Classifies a size change given the `current` and optional `previous`
    /// size in bytes, using the provided `large_threshold` (in bytes) to
    /// distinguish large from small changes.
    pub fn classify(current: Option<u64>, previous: Option<u64>, large_threshold: u64) -> Self {
        let trend = match (current, previous) {
            (Some(_), None) => SizeChangeTrend::New,
            (None, Some(_)) => SizeChangeTrend::Deleted,
            (None, None) => SizeChangeTrend::Unchanged,
            (Some(cur), Some(prev)) => {
                let delta = cur as i64 - prev as i64;
                let abs_delta = delta.unsigned_abs();
                if abs_delta == 0 {
                    SizeChangeTrend::Unchanged
                } else if delta > 0 {
                    if abs_delta >= large_threshold {
                        SizeChangeTrend::IncreasedLarge
                    } else {
                        SizeChangeTrend::IncreasedSmall
                    }
                } else if abs_delta >= large_threshold {
                    SizeChangeTrend::DecreasedLarge
                } else {
                    SizeChangeTrend::DecreasedSmall
                }
            }
        };

        let delta_bytes = match (current, previous) {
            (Some(cur), Some(prev)) => cur as i64 - prev as i64,
            (Some(cur), None) => cur as i64,
            (None, Some(prev)) => -(prev as i64),
            (None, None) => 0,
        };

        Self {
            hex_color: trend.hex_color().to_string(),
            trend,
            delta_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const THRESHOLD: u64 = 1_048_576; // 1 MiB

    #[test]
    fn test_new_item() {
        let sc = SizeChange::classify(Some(500), None, THRESHOLD);
        assert_eq!(sc.trend, SizeChangeTrend::New);
        assert_eq!(sc.delta_bytes, 500);
        assert_eq!(sc.hex_color, "#4CAF50");
    }

    #[test]
    fn test_deleted_item() {
        let sc = SizeChange::classify(None, Some(1000), THRESHOLD);
        assert_eq!(sc.trend, SizeChangeTrend::Deleted);
        assert_eq!(sc.delta_bytes, -1000);
    }

    #[test]
    fn test_unchanged() {
        let sc = SizeChange::classify(Some(1000), Some(1000), THRESHOLD);
        assert_eq!(sc.trend, SizeChangeTrend::Unchanged);
        assert_eq!(sc.delta_bytes, 0);
    }

    #[test]
    fn test_increased_small() {
        let sc = SizeChange::classify(Some(1000 + 100), Some(1000), THRESHOLD);
        assert_eq!(sc.trend, SizeChangeTrend::IncreasedSmall);
        assert_eq!(sc.delta_bytes, 100);
    }

    #[test]
    fn test_increased_large() {
        let sc = SizeChange::classify(Some(1000 + THRESHOLD), Some(1000), THRESHOLD);
        assert_eq!(sc.trend, SizeChangeTrend::IncreasedLarge);
        assert_eq!(sc.delta_bytes, THRESHOLD as i64);
    }

    #[test]
    fn test_decreased_small() {
        let sc = SizeChange::classify(Some(1000), Some(1000 + 100), THRESHOLD);
        assert_eq!(sc.trend, SizeChangeTrend::DecreasedSmall);
        assert_eq!(sc.delta_bytes, -100);
    }

    #[test]
    fn test_decreased_large() {
        let sc = SizeChange::classify(Some(1000), Some(1000 + THRESHOLD), THRESHOLD);
        assert_eq!(sc.trend, SizeChangeTrend::DecreasedLarge);
        assert_eq!(sc.delta_bytes, -(THRESHOLD as i64));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let sc = SizeChange::classify(Some(2_000_000), Some(500_000), THRESHOLD);
        let json = serde_json::to_string(&sc).expect("serialization failed");
        let restored: SizeChange = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(sc, restored);
    }
}