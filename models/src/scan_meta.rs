//! Scan session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata describing a single disk scan session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Unique identifier for this scan session.
    pub id: i64,

    /// Root path that was scanned.
    pub root_path: String,

    /// Timestamp when the scan was started.
    pub started_at: DateTime<Utc>,

    /// Timestamp when the scan completed; `None` if still in progress.
    pub completed_at: Option<DateTime<Utc>>,

    /// Total number of files discovered during the scan.
    pub total_files: u64,

    /// Total number of directories discovered during the scan.
    pub total_dirs: u64,

    /// Total size in bytes of all scanned items.
    pub total_size: u64,

    /// Whether the scan completed without errors.
    pub success: bool,
}

impl ScanMeta {
    /// Creates a new [`ScanMeta`] for the given scan `id` and `root_path`,
    /// recording `started_at` as the provided timestamp.
    pub fn new(id: i64, root_path: impl Into<String>, started_at: DateTime<Utc>) -> Self {
        Self {
            id,
            root_path: root_path.into(),
            started_at,
            completed_at: None,
            total_files: 0,
            total_dirs: 0,
            total_size: 0,
            success: false,
        }
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
    fn new_scan_meta_is_not_complete() {
        let now = Utc::now();
        let meta = ScanMeta::new(1, "/home", now);
        assert_eq!(meta.completed_at, None);
        assert!(!meta.success);
        assert_eq!(meta.duration_secs(), None);
    }
}