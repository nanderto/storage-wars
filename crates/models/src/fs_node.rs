/// In-memory filesystem tree node representing a file or directory.
#[derive(Clone, Debug, PartialEq)]
pub struct FsNode {
    /// Display name of the file or directory.
    pub name: String,
    /// Full path on disk.
    pub path: String,
    /// Size in bytes (sum of all descendants for directories).
    pub size: u64,
    /// Size in bytes from the previous scan, if available.
    pub prev_size: Option<u64>,
    /// Number of files contained (recursively) if this is a directory.
    pub file_count: u64,
    /// Number of folders contained (recursively) if this is a directory.
    pub folder_count: u64,
    /// Last-modified timestamp as an ISO-8601 string.
    pub modified: String,
    /// Whether this node represents a directory.
    pub is_dir: bool,
    /// Child nodes (empty for files).
    pub children: Vec<FsNode>,
}