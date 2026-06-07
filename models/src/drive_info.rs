//! Drive / volume information.

use serde::{Deserialize, Serialize};

/// Information about a storage drive or volume.
///
/// Provides the drive name, optional volume label, and space statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    /// System name of the drive (e.g. `"C:"` on Windows, `"/dev/sda1"` on Linux).
    pub name: String,

    /// Human-readable volume label, if one is assigned.
    pub volume_label: Option<String>,

    /// Total capacity of the drive in bytes.
    pub total_space: u64,

    /// Available (free) space on the drive in bytes.
    pub available_space: u64,
}

impl DriveInfo {
    /// Creates a new `DriveInfo` instance.
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

    /// Returns the used space in bytes.
    pub fn used_space(&self) -> u64 {
        self.total_space.saturating_sub(self.available_space)
    }

    /// Returns the percentage of space used as a value between `0.0` and `100.0`.
    ///
    /// Returns `0.0` if `total_space` is zero to avoid division by zero.
    pub fn used_percent(&self) -> f64 {
        if self.total_space == 0 {
            return 0.0;
        }
        (self.used_space() as f64 / self.total_space as f64) * 100.0
    }

    /// Returns the display label: the volume label if present, otherwise the drive name.
    pub fn display_label(&self) -> &str {
        self.volume_label.as_deref().unwrap_or(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_used_space() {
        let drive = DriveInfo::new("C:", Some("System".to_string()), 1_000_000, 400_000);
        assert_eq!(drive.used_space(), 600_000);
    }

    #[test]
    fn test_used_percent() {
        let drive = DriveInfo::new("C:", None, 1_000_000, 250_000);
        assert!((drive.used_percent() - 75.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_used_percent_zero_total() {
        let drive = DriveInfo::new("C:", None, 0, 0);
        assert_eq!(drive.used_percent(), 0.0);
    }

    #[test]
    fn test_display_label_with_volume_label() {
        let drive = DriveInfo::new("C:", Some("My Drive".to_string()), 1000, 500);
        assert_eq!(drive.display_label(), "My Drive");
    }

    #[test]
    fn test_display_label_without_volume_label() {
        let drive = DriveInfo::new("/dev/sda1", None, 1000, 500);
        assert_eq!(drive.display_label(), "/dev/sda1");
    }
}