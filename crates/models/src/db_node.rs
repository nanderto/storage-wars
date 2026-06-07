/// Flat database representation of a filesystem node.
#[derive(Clone, Debug, PartialEq)]
pub struct DbNode {
    /// Row id in the database.
    pub id: i64,
    /// Parent node id (`None` for root entries).
    pub parent_id: Option<i64>,
    /// Scan session this node belongs to.
    pub scan_id: i64,
    /// Display name of the file or directory.
    pub name: String,
    /// Full path on disk.
    pub path: String,
    /// Size in bytes.
    pub size: u64,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Last-modified timestamp as an ISO-8601 string.
    pub modified: String,
}