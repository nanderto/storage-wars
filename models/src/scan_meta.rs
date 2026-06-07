//! Scan session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents metadata about a single disk scan session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Unique identifier for this scan session.
    pub id: i64,

    /// The root path that was scanned.
    pub root_path: String,

    /// Timestamp when the scan was started.
    pub started_at: DateTime<Utc>,

    /// Timestamp when the scan completed; `None` if still in progress.
    pub completed_at: Option<DateTime<Utc>>,

    /// Total number of files discovered during the scan.
    pub total_files: u64,

    /// Total number of folders discovered during the scan.
    pub total_folders: u64,

    /// Total size in bytes of all scanned entries.
    pub total_size: u64,

    /// Whether the scan completed successfully.
    pub success: bool,

    /// Optional error message if the scan encountered a fatal error.
    pub error_message: Option<String>,
}

impl ScanMeta {
    /// Creates a new [`ScanMeta`] representing a scan that has just started.
    pub fn new(id: i64, root_path: impl Into<String>) -> Self {
        Self {
            id,
            root_path: root_path.into(),
            started_at: Utc::now(),
            completed_at: None,
            total_files: 0,
            total_folders: 0,
            total_size: 0,
            success: false,
            error_message: None,
        }
    }

    /// Returns `true` if the scan is still in progress.
    pub fn is_in_progress(&self) -> bool {
        self.completed_at.is_none()
    }

    /// Returns the duration of the scan in seconds, if it has completed.
    pub fn duration_secs(&self) -> Option<i64> {
        self.completed_at
            .map(|end| (end - self.started_at).num_seconds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_scan_meta_is_in_progress() {
        let meta = ScanMeta::new(1, "/home/user");
        assert!(meta.is_in_progress());
        assert!(!meta.success);
        assert_eq!(meta.root_path, "/home/user");
    }

    #[test]
    fn completed_scan_is_not_in_progress() {
        let mut meta = ScanMeta::new(1, "/home/user");
        meta.completed_at = Some(Utc::now());
        assert!(!meta.is_in_progress());
    }
}