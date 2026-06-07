//! Data-transfer types shared between the database layer and callers.

use serde::{Deserialize, Serialize};

/// A single node (file or directory) as stored in the `nodes` table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DbNode {
    /// Primary key.
    pub id: i64,
    /// Foreign key → `scans.id`.
    pub scan_id: i64,
    /// Absolute path of this entry.
    pub path: String,
    /// `true` if this entry is a directory.
    pub is_dir: bool,
    /// Size in bytes (`0` for directories).
    pub size_bytes: i64,
    /// Optional parent node id; `None` for the root.
    pub parent_id: Option<i64>,
    /// Depth from the scan root (root = 0).
    pub depth: i32,
}

/// Metadata about a completed scan as stored in the `scans` table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Primary key.
    pub id: i64,
    /// Drive letter or mount-point root (e.g. `"C:\\"` or `"/"`).
    pub drive: String,
    /// Absolute path that was scanned.
    pub root_path: String,
    /// Unix timestamp (seconds) when the scan was created.
    pub created_at: i64,
    /// Total number of nodes recorded in this scan.
    pub node_count: i64,
}