//! Scan session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata describing a single scan session.
///
/// Each scan session targets a specific root path and records timing
/// information along with aggregate statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Unique identifier for this scan session.
    pub id: i64,

    /// The root path that was scanned.
    pub root_path: String,

    /// Timestamp when the scan was started.
    pub started_at: DateTime<Utc>,

    /// Timestamp when the scan completed. `None` if the scan is still in progress.
    pub completed_at: Option<DateTime<Utc>>,

    /// Total number of files discovered during the scan.
    pub total_files: u64,

    /// Total number of folders discovered during the scan.
    pub total_folders: u64,

    /// Total size in bytes of all scanned items.
    pub total_size: u64,

    /// Whether the scan completed successfully.
    pub success: bool,

    /// Optional error message if the scan encountered a fatal error.
    pub error_message: Option<String>,
}

impl ScanMeta {
    /// Creates a new `ScanMeta` for a scan that has just started.
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

    /// Marks the scan as successfully completed at the current time.
    pub fn mark_complete(&mut self) {
        self.completed_at = Some(Utc::now());
        self.success = true;
    }

    /// Marks the scan as failed with the given error message.
    pub fn mark_failed(&mut self, error: impl Into<String>) {
        self.completed_at = Some(Utc::now());
        self.success = false;
        self.error_message = Some(error.into());
    }

    /// Returns the duration of the scan in seconds, if it has completed.
    pub fn duration_secs(&self) -> Option<f64> {
        self.completed_at.map(|end| {
            (end - self.started_at)
                .num_milliseconds()
                .max(0) as f64
                / 1000.0
        })
    }

    /// Returns `true` if the scan is still in progress.
    pub fn is_in_progress(&self) -> bool {
        self.completed_at.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scan_meta() {
        let meta = ScanMeta::new(1, "/home/user");
        assert_eq!(meta.id, 1);
        assert_eq!(meta.root_path, "/home/user");
        assert!(meta.is_in_progress());
        assert!(!meta.success);
        assert!(meta.error_message.is_none());
    }

    #[test]
    fn test_mark_complete() {
        let mut meta = ScanMeta::new(1, "/home/user");
        meta.mark_complete();
        assert!(!meta.is_in_progress());
        assert!(meta.success);
        assert!(meta.completed_at.is_some());
    }

    #[test]
    fn test_mark_failed() {
        let mut meta = ScanMeta::new(1, "/home/user");
        meta.mark_failed("Permission denied");
        assert!(!meta.is_in_progress());
        assert!(!meta.success);
        assert_eq!(meta.error_message.as_deref(), Some("Permission denied"));
    }

    #[test]
    fn test_duration_none_when_in_progress() {
        let meta = ScanMeta::new(1, "/");
        assert!(meta.duration_secs().is_none());
    }
}