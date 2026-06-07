/// Flat database representation of a filesystem node.
#[derive(Clone, Debug, PartialEq)]
pub struct DbNode {
    /// Unique row identifier.
    pub id: i64,
    /// Foreign key to the parent node (`None` for root).
    pub parent_id: Option<i64>,
    /// Foreign key to the owning scan session.
    pub scan_id: i64,
    /// Display name.
    pub name: String,
    /// Full path on disk.
    pub path: String,
    /// Size in bytes.
    pub size: u64,
    /// `true` if this node represents a directory.
    pub is_dir: bool,
    /// Last-modified timestamp as an ISO-8601 string.
    pub modified: String,
}