//! Plain-data types shared between the database layer and callers.

use serde::{Deserialize, Serialize};

/// Metadata for a single completed scan (maps to the `scans` table).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Primary key.
    pub id: i64,
    /// Drive / volume root that was scanned (e.g. `"C:\\"` or `"/"`).
    pub drive_root: String,
    /// ISO-8601 timestamp stored as text.
    pub scanned_at: String,
    /// Total number of nodes recorded for this scan.
    pub node_count: i64,
}

/// A single file-system node as stored in the `nodes` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNode {
    /// Primary key.
    pub id: i64,
    /// Foreign key → `scans.id`.
    pub scan_id: i64,
    /// Absolute path of this node.
    pub path: String,
    /// `true` for directories, `false` for files.
    pub is_dir: bool,
    /// Size in bytes (`0` for directories).
    pub size_bytes: i64,
    /// `id` of the parent node, or `None` for the root.
    pub parent_id: Option<i64>,
    /// Depth from the scan root (root = 0).
    pub depth: i32,
}

/// A node supplied by the caller when saving a new scan.
///
/// `parent_id` is expressed in terms of the *caller's* temporary IDs and is
/// resolved to real database IDs inside [`crate::db::save_scan`].
#[derive(Debug, Clone)]
pub struct ScanNode {
    /// Caller-assigned temporary ID (used to express parent–child links).
    pub temp_id: i64,
    /// Caller-assigned temporary parent ID (`None` for the root).
    pub temp_parent_id: Option<i64>,
    /// Absolute path of this node.
    pub path: String,
    /// `true` for directories, `false` for files.
    pub is_dir: bool,
    /// Size in bytes.
    pub size_bytes: i64,
    /// Depth from the scan root.
    pub depth: i32,
}