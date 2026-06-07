/// Represents a node in the filesystem tree (file or folder).
#[derive(Clone, Debug, PartialEq)]
pub struct FsNode {
    /// Display name of the file or folder.
    pub name: String,
    /// Full path on disk.
    pub path: String,
    /// Size in bytes for the current scan.
    pub size: u64,
    /// Size in bytes from the previous scan, if available.
    pub prev_size: Option<u64>,
    /// Number of files contained (recursive for folders, 1 for a file).
    pub file_count: u64,
    /// Number of folders contained (recursive, 0 for a file).
    pub folder_count: u64,
    /// Last-modified timestamp as an ISO-8601 string.
    pub modified: String,
    /// `true` if this node represents a directory.
    pub is_dir: bool,
    /// Child nodes (empty for files).
    pub children: Vec<FsNode>,
}