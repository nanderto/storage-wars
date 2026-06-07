//! Drive / volume information.

use serde::{Deserialize, Serialize};

/// Information about a storage drive or volume.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    /// System drive name (e.g. `"C:"` on Windows, `"/dev/sda1"` on Linux).
    pub name: String,

    /// Human-readable volume label assigned to the drive, if any.
    pub volume_label: Option<String>,

    /// Total capacity of the drive in bytes.
    pub total_space: u64,

    /// Currently available (free) space on the drive in bytes.
    pub available_space: u64,
}

impl DriveInfo {
    /// Creates a new [`DriveInfo`] with the given name and space values.
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
    /// Saturates at zero to guard against inconsistent OS-reported values.
    pub fn used_space(&self) -> u64 {
        self.total_space.saturating_sub(self.available_space)
    }

    /// Returns the percentage of space used as a value in `[0.0, 100.0]`.
    /// Returns `0.0` when `total_space` is zero.
    pub fn used_percent(&self) -> f64 {
        if self.total_space == 0 {
            return 0.0;
        }
        (self.used_space() as f64 / self.total_space as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn used_space_is_correct() {
        let drive = DriveInfo::new("C:", Some("System".to_string()), 1_000, 400);
        assert_eq!(drive.used_space(), 600);
    }

    #[test]
    fn used_percent_is_correct() {
        let drive = DriveInfo::new("C:", None, 1_000, 250);
        assert!((drive.used_percent() - 75.0).abs() < f64::EPSILON);
    }

    #[test]
    fn used_percent_zero_when_total_is_zero() {
        let drive = DriveInfo::new("C:", None, 0, 0);
        assert_eq!(drive.used_percent(), 0.0);
    }
}