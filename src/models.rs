//! Plain data structures shared between the database layer and callers.

use serde::{Deserialize, Serialize};

/// A single file-system node as stored in the `nodes` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNode {
    /// Primary key.
    pub id: i64,
    /// Foreign key → `scans.id`.
    pub scan_id: i64,
    /// Optional parent node id (`None` for the root).
    pub parent_id: Option<i64>,
    /// Absolute path of this node.
    pub path: String,
    /// `true` if this node is a directory.
    pub is_dir: bool,
    /// Size in bytes (`0` for directories unless pre-computed).
    pub size_bytes: i64,
}

/// Lightweight metadata about a completed scan, returned by
/// [`get_scans_for_drive`](crate::get_scans_for_drive).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanMeta {
    /// Primary key.
    pub id: i64,
    /// Drive / volume root that was scanned (e.g. `C:\` or `/`).
    pub drive_root: String,
    /// RFC-3339 timestamp string stored in the database.
    pub scanned_at: String,
    /// Total number of nodes recorded for this scan.
    pub node_count: i64,
}

/// An in-memory node used as input to
/// [`save_scan`](crate::save_scan).
///
/// The tree is expressed as a flat `Vec` where each node carries its
/// own children, enabling depth-first traversal during insertion.
#[derive(Debug, Clone)]
pub struct ScanNode {
    /// Absolute path of this node.
    pub path: String,
    /// `true` if this node is a directory.
    pub is_dir: bool,
    /// Size in bytes.
    pub size_bytes: i64,
    /// Child nodes (empty for files).
    pub children: Vec<ScanNode>,
}