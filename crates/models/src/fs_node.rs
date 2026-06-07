/// Represents a node in the in-memory filesystem tree.
#[derive(Clone, Debug, PartialEq)]
pub struct FsNode {
    /// Display name of the file or directory.
    pub name: String,
    /// Full path on disk.
    pub path: String,
    /// Size in bytes (aggregated for directories).
    pub size: u64,
    /// Size from the previous scan, used for delta comparison.
    pub prev_size: u64,
    /// Number of files contained (recursively for directories).
    pub file_count: u64,
    /// Number of folders contained (recursively for directories).
    pub folder_count: u64,
    /// Last-modified timestamp as an ISO-8601 string.
    pub modified: String,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Child nodes (empty for files).
    pub children: Vec<FsNode>,
}