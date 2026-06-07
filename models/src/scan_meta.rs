//! Scan session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the metadata for a single disk scan session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Unique identifier for this scan session.
    pub id: i64,

    /// The root path that was scanned.
    pub root_path: String,

    /// Timestamp when the scan was started.
    pub started_at: DateTime<Utc>,

    /// Timestamp when the scan completed. `None` if still in progress.
    pub completed_at: Option<DateTime<Utc>>,

    /// Total number of files discovered during the scan.
    pub total_files: u64,

    /// Total number of folders discovered during the scan.
    pub total_folders: u64,

    /// Total size in bytes of all scanned items.
    pub total_size: u64,

    /// Whether the scan completed successfully.
    pub is_complete: bool,

    /// Optional error message if the scan encountered a fatal error.
    pub error: Option<String>,
}

impl ScanMeta {
    /// Creates a new `ScanMeta` for a scan that is starting now.
    pub fn new(id: i64, root_path: impl Into<String>) -> Self {
        Self {
            id,
            root_path: root_path.into(),
            started_at: Utc::now(),
            completed_at: None,
            total_files: 0,
            total_folders: 0,
            total_size: 0,
            is_complete: false,
            error: None,
        }
    }

    /// Marks the scan as complete at the current time.
    pub fn mark_complete(&mut self) {
        self.completed_at = Some(Utc::now());
        self.is_complete = true;
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
    fn test_new_scan_meta() {
        let meta = ScanMeta::new(1, "/home/user");
        assert_eq!(meta.id, 1);
        assert_eq!(meta.root_path, "/home/user");
        assert!(!meta.is_complete);
        assert!(meta.completed_at.is_none());
    }

    #[test]
    fn test_mark_complete() {
        let mut meta = ScanMeta::new(1, "/home/user");
        meta.mark_complete();
        assert!(meta.is_complete);
        assert!(meta.completed_at.is_some());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let meta = ScanMeta::new(7, "/tmp");
        let json = serde_json::to_string(&meta).expect("serialization failed");
        let restored: ScanMeta = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(meta, restored);
    }
}