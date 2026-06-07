/// Represents a node in the filesystem tree (recursive).
#[derive(Debug, Clone)]
pub struct FsNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub file_count: u64,
    pub folder_count: u64,
    pub modified: Option<String>,
    pub children: Vec<FsNode>,
}

/// Flat database representation of a filesystem node, with parent reference.
#[derive(Debug, Clone)]
pub struct DbNode {
    pub id: Option<i64>,
    pub scan_id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub file_count: u64,
    pub folder_count: u64,
    pub modified: Option<String>,
}

/// Metadata about a single scan session.
#[derive(Debug, Clone)]
pub struct ScanMeta {
    pub scan_id: i64,
    pub drive_name: String,
    pub scanned_at: String,
    pub total_size: u64,
}