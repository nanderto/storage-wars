//! Drive / volume information.

use serde::{Deserialize, Serialize};

/// Describes a storage drive or volume available on the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriveInfo {
    /// Human-readable name of the drive (e.g. `"Local Disk"`, `"USB Drive"`).
    pub name: String,

    /// Volume label assigned to the drive, if any.
    pub volume_label: Option<String>,

    /// Mount point or drive letter (e.g. `"C:\\"` on Windows, `"/"` on Unix).
    pub mount_point: String,

    /// Total capacity of the drive in bytes.
    pub total_space: u64,

    /// Available (free) space on the drive in bytes.
    pub available_space: u64,
}

impl DriveInfo {
    /// Creates a new [`DriveInfo`] with the given name, mount point, and
    /// space values.
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
    ///
    /// Returns `0` if `available_space` exceeds `total_space` to avoid
    /// underflow on unexpected data.
    pub fn used_space(&self) -> u64 {
        self.total_space.saturating_sub(self.available_space)
    }

    /// Returns the percentage of space used as a value in `[0.0, 100.0]`.
    ///
    /// Returns `0.0` if `total_space` is zero.
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
    fn test_new_drive_info() {
        let drive = DriveInfo::new("Local Disk", "C:\\", 500_000_000_000, 200_000_000_000);
        assert_eq!(drive.name, "Local Disk");
        assert_eq!(drive.mount_point, "C:\\");
        assert_eq!(drive.total_space, 500_000_000_000);
        assert_eq!(drive.available_space, 200_000_000_000);
        assert!(drive.volume_label.is_none());
    }

    #[test]
    fn test_used_space() {
        let drive = DriveInfo::new("Disk", "/", 1000, 400);
        assert_eq!(drive.used_space(), 600);
    }

    #[test]
    fn test_used_space_no_underflow() {
        let drive = DriveInfo::new("Disk", "/", 100, 200);
        assert_eq!(drive.used_space(), 0);
    }

    #[test]
    fn test_used_percent() {
        let drive = DriveInfo::new("Disk", "/", 1000, 250);
        assert!((drive.used_percent() - 75.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_used_percent_zero_total() {
        let drive = DriveInfo::new("Empty", "/mnt/empty", 0, 0);
        assert_eq!(drive.used_percent(), 0.0);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut drive = DriveInfo::new("Data", "/data", 2_000_000_000, 500_000_000);
        drive.volume_label = Some("DATA_VOL".to_string());

        let json = serde_json::to_string(&drive).expect("serialization failed");
        let restored: DriveInfo = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(drive, restored);
    }
}