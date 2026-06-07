//! Scan session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents metadata for a single scan session, capturing when the scan
/// occurred, which path was scanned, and aggregate statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Unique identifier for this scan session.
    pub id: i64,

    /// The root path that was scanned.
    pub root_path: String,

    /// The timestamp when the scan was started.
    pub started_at: DateTime<Utc>,

    /// The timestamp when the scan completed. `None` if still in progress.
    pub completed_at: Option<DateTime<Utc>>,

    /// Total number of files discovered during the scan.
    pub total_files: u64,

    /// Total number of folders discovered during the scan.
    pub total_folders: u64,

    /// Total size in bytes of all scanned items.
    pub total_size: u64,
}

impl ScanMeta {
    /// Creates a new [`ScanMeta`] for a scan that has just started.
    pub fn new(id: i64, root_path: impl Into<String>, started_at: DateTime<Utc>) -> Self {
        Self {
            id,
            root_path: root_path.into(),
            started_at,
            completed_at: None,
            total_files: 0,
            total_folders: 0,
            total_size: 0,
        }
    }

    /// Returns `true` if the scan has completed.
    pub fn is_complete(&self) -> bool {
        self.completed_at.is_some()
    }

    /// Marks the scan as complete with the given timestamp.
    pub fn mark_complete(&mut self, completed_at: DateTime<Utc>) {
        self.completed_at = Some(completed_at);
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
    fn new_scan_is_not_complete() {
        let meta = ScanMeta::new(1, "/home", Utc::now());
        assert!(!meta.is_complete());
    }

    #[test]
    fn mark_complete_sets_completed_at() {
        let mut meta = ScanMeta::new(1, "/home", Utc::now());
        meta.mark_complete(Utc::now());
        assert!(meta.is_complete());
    }
}