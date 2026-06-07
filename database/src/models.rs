//! Plain-data structs that mirror the database rows.

use serde::{Deserialize, Serialize};

/// A single node (file or directory) stored in the `nodes` table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbNode {
    /// Primary key — `None` before the row has been inserted.
    pub id: Option<i64>,
    /// Foreign key referencing `scans.id`.
    pub scan_id: i64,
    /// Optional parent node id; `None` for the root node.
    pub parent_id: Option<i64>,
    /// Absolute path of this entry.
    pub path: String,
    /// `true` if this entry is a directory.
    pub is_dir: bool,
    /// File size in bytes; `0` for directories.
    pub size: i64,
    /// Last-modified timestamp as a Unix epoch (seconds).
    pub modified: i64,
}

/// Metadata about a completed scan stored in the `scans` table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Primary key.
    pub id: i64,
    /// The drive / root path that was scanned (e.g. `C:\` or `/`).
    pub drive_root: String,
    /// When the scan was started, as a Unix epoch (seconds).
    pub scanned_at: i64,
    /// Total number of nodes recorded for this scan.
    pub node_count: i64,
}