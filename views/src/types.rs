//! Shared domain types used across view components.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Drive / volume
// ---------------------------------------------------------------------------

/// Represents a mounted drive or volume available for scanning.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    /// Unique identifier for this drive entry.
    pub id: Uuid,
    /// OS-level mount path (e.g. `C:\` on Windows, `/dev/sda1` on Linux).
    pub path: String,
    /// Optional human-readable volume label.
    pub volume_label: Option<String>,
    /// Total capacity in bytes.
    pub total_bytes: u64,
    /// Available (free) bytes.
    pub available_bytes: u64,
}

impl DriveInfo {
    /// Formats a display label: `"Label (C:\) — 42.3 GB free"` or
    /// `"C:\ — 42.3 GB free"` when no volume label is present.
    pub fn display_label(&self) -> String {
        let space = bytesize::ByteSize(self.available_bytes).to_string_as(true);
        match &self.volume_label {
            Some(label) => format!("{} ({}) — {} free", label, self.path, space),
            None => format!("{} — {} free", self.path, space),
        }
    }
}

// ---------------------------------------------------------------------------
// Scan / history
// ---------------------------------------------------------------------------

/// A single completed scan snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanSnapshot {
    pub id: Uuid,
    pub drive_id: Uuid,
    pub scanned_at: DateTime<Utc>,
    pub root: FileNode,
}

// ---------------------------------------------------------------------------
// File tree
// ---------------------------------------------------------------------------

/// Colour-coded change indicator between two scan snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SizeChange {
    /// Entry is new (not present in base snapshot).
    New,
    /// Entry was deleted (not present in new snapshot).
    Deleted,
    /// Size increased significantly.
    Grew,
    /// Size decreased significantly.
    Shrank,
    /// Size is roughly unchanged.
    Unchanged,
}

impl Default for SizeChange {
    fn default() -> Self {
        Self::Unchanged
    }
}

/// A node in the scanned file-system tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileNode {
    pub id: Uuid,
    /// Display name (file or directory name, not full path).
    pub name: String,
    /// `true` if this node represents a directory.
    pub is_dir: bool,
    /// Size in bytes for this node (inclusive of children for directories).
    pub size_bytes: u64,
    /// Size in bytes from the previous (base) snapshot, if available.
    pub prev_size_bytes: Option<u64>,
    /// Number of files directly or transitively contained.
    pub file_count: u64,
    /// Number of directories directly or transitively contained.
    pub folder_count: u64,
    /// Last-modified timestamp.
    pub modified_at: Option<DateTime<Utc>>,
    /// Fraction of the parent node's size, in `[0.0, 1.0]`.
    pub parent_fraction: f32,
    /// Change classification vs. the base snapshot.
    pub size_change: SizeChange,
    /// Child nodes (empty for files).
    pub children: Vec<FileNode>,
    /// Whether this node is currently expanded in the tree view.
    #[serde(skip)]
    pub expanded: bool,
}

impl FileNode {
    /// Returns the percentage change relative to `prev_size_bytes`, or `None`.
    pub fn percent_change(&self) -> Option<f64> {
        let prev = self.prev_size_bytes?;
        if prev == 0 {
            return None;
        }
        let delta = self.size_bytes as f64 - prev as f64;
        Some(delta / prev as f64 * 100.0)
    }
}

// ---------------------------------------------------------------------------
// Column definitions
// ---------------------------------------------------------------------------

/// Columns rendered by [`crate::TreeView`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeColumn {
    Name,
    PercentParent,
    Size,
    PrevSize,
    PercentPrev,
    Files,
    Folders,
    Modified,
}

impl TreeColumn {
    pub const ALL: &'static [TreeColumn] = &[
        TreeColumn::Name,
        TreeColumn::PercentParent,
        TreeColumn::Size,
        TreeColumn::PrevSize,
        TreeColumn::PercentPrev,
        TreeColumn::Files,
        TreeColumn::Folders,
        TreeColumn::Modified,
    ];

    pub fn header(&self) -> &'static str {
        match self {
            TreeColumn::Name => "Name",
            TreeColumn::PercentParent => "% Parent",
            TreeColumn::Size => "Size",
            TreeColumn::PrevSize => "Prev Size",
            TreeColumn::PercentPrev => "% Prev",
            TreeColumn::Files => "Files",
            TreeColumn::Folders => "Folders",
            TreeColumn::Modified => "Modified",
        }
    }

    pub fn default_width(&self) -> f32 {
        match self {
            TreeColumn::Name => 260.0,
            TreeColumn::PercentParent => 90.0,
            TreeColumn::Size => 90.0,
            TreeColumn::PrevSize => 90.0,
            TreeColumn::PercentPrev => 80.0,
            TreeColumn::Files => 70.0,
            TreeColumn::Folders => 70.0,
            TreeColumn::Modified => 140.0,
        }
    }
}