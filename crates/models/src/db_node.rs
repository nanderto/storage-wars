/// Flat database representation of a filesystem node.
#[derive(Clone, Debug, PartialEq)]
pub struct DbNode {
    /// Unique row identifier.
    pub id: i64,
    /// Reference to the parent node's id (None for root).
    pub parent_id: Option<i64>,
    /// Reference to the scan session this node belongs to.
    pub scan_id: i64,
    /// Display name of the file or directory.
    pub name: String,
    /// Full path on disk.
    pub path: String,
    /// Size in bytes.
    pub size: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Number of files contained (recursively).
    pub file_count: u64,
    /// Number of folders contained (recursively).
    pub folder_count: u64,
    /// Last-modified timestamp as an ISO-8601 string.
    pub modified: String,
}