//! Drive / volume information.

use serde::{Deserialize, Serialize};

/// Information about a storage drive or volume.
///
/// Captures the human-readable drive name, the volume label reported by the OS,
/// and space statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    /// The drive name as reported by the operating system (e.g. `"C:"` on Windows,
    /// `"/dev/sda1"` on Linux).
    pub name: String,

    /// The user-assigned volume label, if any (e.g. `"System"`, `"Data"`).
    pub volume_label: Option<String>,

    /// The mount point path (e.g. `"C:\\"` or `"/"`).
    pub mount_point: String,

    /// Total capacity of the drive in bytes.
    pub total_space: u64,

    /// Available (free) space on the drive in bytes.
    pub available_space: u64,
}

impl DriveInfo {
    /// Creates a new [`DriveInfo`].
    pub fn new(
        name: impl Into<String>,
        mount_point: impl Into<String>,
        total_space: u64,
        available_space: u64,
    ) -> Self {
        Self {
            name: name.into(),
            volume_label: None,
            mount_point: mount_point.into(),
            total_space,
            available_space,
        }
    }

    /// Returns the used space in bytes (`total_space - available_space`).
    pub fn used_space(&self) -> u64 {
        self.total_space.saturating_sub(self.available_space)
    }

    /// Returns the percentage of space used, in the range `[0.0, 100.0]`.
    ///
    /// Returns `0.0` if `total_space` is zero to avoid division by zero.
    pub fn used_percent(&self) -> f64 {
        if self.total_space == 0 {
            return 0.0;
        }
        (self.used_space() as f64 / self.total_space as f64) * 100.0
    }

    /// Returns the display name: the volume label if set, otherwise the drive name.
    pub fn display_name(&self) -> &str {
        self.volume_label.as_deref().unwrap_or(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn used_space_computed_correctly() {
        let drive = DriveInfo::new("C:", "C:\\", 1_000_000, 400_000);
        assert_eq!(drive.used_space(), 600_000);
    }

    #[test]
    fn used_percent_computed_correctly() {
        let drive = DriveInfo::new("C:", "C:\\", 1_000_000, 250_000);
        assert!((drive.used_percent() - 75.0).abs() < f64::EPSILON);
    }

    #[test]
    fn used_percent_zero_when_total_is_zero() {
        let drive = DriveInfo::new("empty", "/empty", 0, 0);
        assert_eq!(drive.used_percent(), 0.0);
    }

    #[test]
    fn display_name_prefers_volume_label() {
        let mut drive = DriveInfo::new("D:", "D:\\", 500_000, 100_000);
        drive.volume_label = Some("Backup".to_string());
        assert_eq!(drive.display_name(), "Backup");
    }

    #[test]
    fn display_name_falls_back_to_name() {
        let drive = DriveInfo::new("E:", "E:\\", 500_000, 100_000);
        assert_eq!(drive.display_name(), "E:");
    }

    #[test]
    fn serialization_round_trip() {
        let mut drive = DriveInfo::new("/dev/sda1", "/", 500_107_862_016, 120_000_000_000);
        drive.volume_label = Some("root".to_string());
        let json = serde_json::to_string(&drive).unwrap();
        let restored: DriveInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(drive, restored);
    }
}