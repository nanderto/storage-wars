//! Drive / volume information.

use serde::{Deserialize, Serialize};

/// Describes a storage drive or volume, including its label, total capacity,
/// and available free space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    /// The system name of the drive (e.g. `"C:"` on Windows, `"/dev/sda1"` on Linux).
    pub name: String,

    /// The human-readable volume label, if one is set.
    pub volume_label: Option<String>,

    /// Total capacity of the drive in bytes.
    pub total_space: u64,

    /// Available (free) space on the drive in bytes.
    pub available_space: u64,
}

impl DriveInfo {
    /// Creates a new [`DriveInfo`].
    pub fn new(
        name: impl Into<String>,
        volume_label: Option<String>,
        total_space: u64,
        available_space: u64,
    ) -> Self {
        Self {
            name: name.into(),
            volume_label,
            total_space,
            available_space,
        }
    }

    /// Returns the used space in bytes (`total_space - available_space`).
    pub fn used_space(&self) -> u64 {
        self.total_space.saturating_sub(self.available_space)
    }

    /// Returns the usage ratio as a value between `0.0` and `1.0`.
    /// Returns `0.0` if `total_space` is zero.
    pub fn usage_ratio(&self) -> f64 {
        if self.total_space == 0 {
            return 0.0;
        }
        self.used_space() as f64 / self.total_space as f64
    }

    /// Returns the usage percentage (0–100).
    pub fn usage_percent(&self) -> f64 {
        self.usage_ratio() * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn used_space_is_difference() {
        let drive = DriveInfo::new("C:", None, 1_000, 400);
        assert_eq!(drive.used_space(), 600);
    }

    #[test]
    fn usage_ratio_zero_when_total_is_zero() {
        let drive = DriveInfo::new("X:", None, 0, 0);
        assert_eq!(drive.usage_ratio(), 0.0);
    }

    #[test]
    fn usage_percent_correct() {
        let drive = DriveInfo::new("D:", None, 200, 50);
        assert!((drive.usage_percent() - 75.0).abs() < f64::EPSILON);
    }
}