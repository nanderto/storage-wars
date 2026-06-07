//! Shared data types used across view components.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Drive
// ---------------------------------------------------------------------------

/// Represents a mounted drive / volume available for scanning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drive {
    /// Unique identifier (e.g. drive letter on Windows, mount-point on Unix).
    pub id: String,
    /// Mount path (e.g. `C:\` or `/`).
    pub path: String,
    /// Optional human-readable volume label.
    pub volume_label: Option<String>,
    /// Total capacity in bytes.
    pub total_bytes: u64,
    /// Available (free) bytes.
    pub available_bytes: u64,
}

impl Drive {
    /// Returns a formatted label suitable for display in [`DriveSelector`].
    ///
    /// Format: `"<path> (<volume_label>) — <available> free of <total>"`
    pub fn display_label(&self) -> String {
        let vol = self
            .volume_label
            .as_deref()
            .map(|v| format!(" ({v})"))
            .unwrap_or_default();

        let available = bytesize::ByteSize(self.available_bytes);
        let total = bytesize::ByteSize(self.total_bytes);

        format!("{}{vol} — {available} free of {total}", self.path)
    }
}

// ---------------------------------------------------------------------------
// ScanRecord
// ---------------------------------------------------------------------------

/// A persisted record of a completed scan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanRecord {
    pub id: Uuid,
    pub drive_id: String,
    pub drive_path: String,
    pub label: String,
    pub scanned_at: DateTime<Utc>,
    pub total_bytes: u64,
    pub file_count: u64,
    pub folder_count: u64,
}

// ---------------------------------------------------------------------------
// FileNode
// ---------------------------------------------------------------------------

/// A node in the scanned file-system tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub prev_size_bytes: Option<u64>,
    pub file_count: u64,
    pub folder_count: u64,
    pub modified: Option<DateTime<Utc>>,
    pub children: Vec<FileNode>,
    /// Depth in the tree (root = 0).
    pub depth: usize,
    pub is_expanded: bool,
}

impl FileNode {
    /// Percentage of parent's size (0–100).
    pub fn percent_of_parent(&self, parent_bytes: u64) -> f64 {
        if parent_bytes == 0 {
            return 0.0;
        }
        (self.size_bytes as f64 / parent_bytes as f64) * 100.0
    }

    /// Percentage change relative to previous size.
    pub fn percent_change(&self) -> Option<f64> {
        let prev = self.prev_size_bytes?;
        if prev == 0 {
            return None;
        }
        Some(((self.size_bytes as f64 - prev as f64) / prev as f64) * 100.0)
    }

    /// Classify the size change for colour coding.
    pub fn size_change(&self) -> SizeChange {
        match self.percent_change() {
            None => SizeChange::Unchanged,
            Some(p) if p > 10.0 => SizeChange::Grown,
            Some(p) if p < -10.0 => SizeChange::Shrunk,
            _ => SizeChange::Unchanged,
        }
    }
}

// ---------------------------------------------------------------------------
// SizeChange
// ---------------------------------------------------------------------------

/// Colour-coding category for a node's size change between scans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeChange {
    Grown,
    Shrunk,
    Unchanged,
}

// ---------------------------------------------------------------------------
// ScanState
// ---------------------------------------------------------------------------

/// Current state of an in-progress or completed scan.
#[derive(Debug, Clone, PartialEq)]
pub enum ScanState {
    Idle,
    Scanning { progress: f32, current_path: String },
    Complete,
    Error(String),
}

impl Default for ScanState {
    fn default() -> Self {
        Self::Idle
    }
}

// ---------------------------------------------------------------------------
// CompareMode
// ---------------------------------------------------------------------------

/// Which scan record is selected as Base vs New for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionRole {
    #[default]
    None,
    Base,
    New,
}