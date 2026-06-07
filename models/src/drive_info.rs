//! Drive and volume information.

use serde::{Deserialize, Serialize};

/// Information about a storage drive or volume.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    /// The drive name or mount point (e.g. `"C:\\"` on Windows, `"/"` on Unix).
    pub name: String,

    /// The volume label assigned to the drive, if any.
    pub volume_label: Option<String>,

    /// Total capacity of the drive in bytes.
    pub total_space: u64,

    /// Available (free) space on the drive in bytes.
    pub available_space: u64,
}

impl DriveInfo {
    /// Creates a new `DriveInfo` with the given drive details.
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

    /// Returns the percentage of space used as a value between 0.0 and 100.0.
    /// Returns `0.0` if `total_space` is zero.
    pub fn used_percent(&self) -> f64 {
        if self.total_space == 0 {
            return 0.0;
        }
        (self.used_space() as f64 / self.total_space as f64) * 100.0
    }

    /// Returns `true` if the drive has a volume label.
    pub fn has_label(&self) -> bool {
        self.volume_label.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_drive_info() {
        let info = DriveInfo::new("C:\\", Some("System".to_string()), 500_000_000_000, 200_000_000_000);
        assert_eq!(info.name, "C:\\");
        assert_eq!(info.volume_label, Some("System".to_string()));
        assert_eq!(info.total_space, 500_000_000_000);
        assert_eq!(info.available_space, 200_000_000_000);
    }

    #[test]
    fn test_used_space() {
        let info = DriveInfo::new("/", None, 1000, 400);
        assert_eq!(info.used_space(), 600);
    }

    #[test]
    fn test_used_percent() {
        let info = DriveInfo::new("/", None, 1000, 250);
        assert!((info.used_percent() - 75.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_used_percent_zero_total() {
        let info = DriveInfo::new("/", None, 0, 0);
        assert_eq!(info.used_percent(), 0.0);
    }

    #[test]
    fn test_has_label() {
        let with_label = DriveInfo::new("/", Some("Data".to_string()), 1000, 500);
        let without_label = DriveInfo::new("/", None, 1000, 500);
        assert!(with_label.has_label());
        assert!(!without_label.has_label());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let info = DriveInfo::new("D:\\", Some("Backup".to_string()), 2_000_000_000_000, 1_500_000_000_000);
        let json = serde_json::to_string(&info).expect("serialization failed");
        let restored: DriveInfo = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(info, restored);
    }
}