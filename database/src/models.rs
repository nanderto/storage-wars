/// Metadata about a completed scan stored in the `scans` table.
#[derive(Debug, Clone, PartialEq)]
pub struct ScanMeta {
    /// Primary key.
    pub id: i64,
    /// Drive letter or mount point (e.g. `"C"`, `"/mnt/data"`).
    pub drive: String,
    /// Root path that was scanned.
    pub root_path: String,
    /// Unix timestamp (seconds) when the scan was performed.
    pub scanned_at: i64,
    /// Total size in bytes of all nodes in this scan.
    pub total_bytes: i64,
}

/// A single file or directory node stored in the `nodes` table.
#[derive(Debug, Clone, PartialEq)]
pub struct DbNode {
    /// Primary key.
    pub id: i64,
    /// Foreign key referencing `scans.id`.
    pub scan_id: i64,
    /// Absolute path of this node.
    pub path: String,
    /// `true` if this node is a directory.
    pub is_dir: bool,
    /// Size in bytes (0 for directories).
    pub size_bytes: i64,
    /// Optional parent node id (`None` for the root node).
    pub parent_id: Option<i64>,
    /// Depth from the scan root (root = 0).
    pub depth: i64,
}