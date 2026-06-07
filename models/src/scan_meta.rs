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

    /// Timestamp when the scan completed, or `None` if still in progress.
    pub completed_at: Option<DateTime<Utc>>,

    /// Total number of files discovered during the scan.
    pub total_files: u64,

    /// Total number of directories discovered during the scan.
    pub total_dirs: u64,

    /// Total size in bytes of all scanned items.
    pub total_size: u64,

    /// Whether the scan completed successfully.
    pub is_complete: bool,

    /// Optional label or description for this scan session.
    pub label: Option<String>,
}

impl ScanMeta {
    /// Creates a new [`ScanMeta`] for the given scan `id` and `root_path`,
    /// recording the current UTC time as the start time.
    pub fn new(id: i64, root_path: impl Into<String>) -> Self {
        Self {
            id,
            root_path: root_path.into(),
            started_at: Utc::now(),
            completed_at: None,
            total_files: 0,
            total_dirs: 0,
            total_size: 0,
            is_complete: false,
            label: None,
        }
    }

    /// Marks the scan as complete, recording the current UTC time.
    pub fn mark_complete(&mut self) {
        self.completed_at = Some(Utc::now());
        self.is_complete = true;
    }

    /// Returns the duration of the scan in seconds, if it has completed.
    pub fn duration_secs(&self) -> Option<i64> {
        self.completed_at.map(|end| {
            (end - self.started_at).num_seconds()
        })
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
        assert_eq!(meta.total_files, 0);
    }

    #[test]
    fn test_mark_complete() {
        let mut meta = ScanMeta::new(1, "/");
        assert!(!meta.is_complete);
        meta.mark_complete();
        assert!(meta.is_complete);
        assert!(meta.completed_at.is_some());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut meta = ScanMeta::new(2, "/data");
        meta.total_files = 1000;
        meta.total_size = 1_048_576;
        meta.mark_complete();

        let json = serde_json::to_string(&meta).expect("serialization failed");
        let restored: ScanMeta = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(meta.id, restored.id);
        assert_eq!(meta.root_path, restored.root_path);
        assert_eq!(meta.is_complete, restored.is_complete);
        assert_eq!(meta.total_files, restored.total_files);
    }
}