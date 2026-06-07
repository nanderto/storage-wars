//! Scan session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata describing a single disk-scan session.
///
/// Each time the application scans a drive or directory, a [`ScanMeta`] record is
/// created to capture when the scan occurred, which path was scanned, and summary
/// statistics about the results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Unique identifier for this scan session.
    pub id: i64,

    /// The root path that was scanned.
    pub root_path: String,

    /// When the scan was started.
    pub started_at: DateTime<Utc>,

    /// When the scan completed, or `None` if it is still in progress.
    pub completed_at: Option<DateTime<Utc>>,

    /// Total number of files discovered during the scan.
    pub total_files: u64,

    /// Total number of folders discovered during the scan.
    pub total_folders: u64,

    /// Total size in bytes of all scanned items.
    pub total_size: u64,

    /// Whether the scan completed successfully.
    pub success: bool,

    /// An optional error message if the scan did not complete successfully.
    pub error_message: Option<String>,
}

impl ScanMeta {
    /// Creates a new [`ScanMeta`] for a scan that is starting now.
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

    /// Returns the duration of the scan in seconds, or `None` if not yet complete.
    pub fn duration_secs(&self) -> Option<i64> {
        self.completed_at
            .map(|end| (end - self.started_at).num_seconds())
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
    fn new_scan_is_in_progress() {
        let meta = ScanMeta::new(1, "/home");
        assert!(meta.is_in_progress());
        assert!(!meta.success);
        assert!(meta.completed_at.is_none());
    }

    #[test]
    fn mark_complete_sets_success() {
        let mut meta = ScanMeta::new(1, "/home");
        meta.mark_complete();
        assert!(meta.success);
        assert!(!meta.is_in_progress());
        assert!(meta.completed_at.is_some());
    }

    #[test]
    fn mark_failed_sets_error() {
        let mut meta = ScanMeta::new(1, "/home");
        meta.mark_failed("permission denied");
        assert!(!meta.success);
        assert_eq!(meta.error_message.as_deref(), Some("permission denied"));
        assert!(!meta.is_in_progress());
    }

    #[test]
    fn serialization_round_trip() {
        let mut meta = ScanMeta::new(7, "/mnt/data");
        meta.total_files = 1000;
        meta.total_size = 1_000_000;
        let json = serde_json::to_string(&meta).unwrap();
        let restored: ScanMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(meta, restored);
    }
}