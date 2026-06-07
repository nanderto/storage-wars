/// Metadata for a single scan session.
#[derive(Clone, Debug, PartialEq)]
pub struct ScanMeta {
    pub id: i64,
    pub drive: String,
    pub timestamp: String,
    pub total_size: u64,
}

/// Represents a node in the filesystem tree.
#[derive(Clone, Debug, PartialEq)]
pub struct FsNode {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub prev_size: u64,
    pub file_count: u64,
    pub folder_count: u64,
    pub modified: String,
    pub is_dir: bool,
    pub children: Vec<FsNode>,
}

/// Flat database representation of a filesystem node.
#[derive(Clone, Debug, PartialEq)]
pub struct DbNode {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub scan_id: i64,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub modified: String,
}

/// Information about a drive / volume.
#[derive(Clone, Debug, PartialEq)]
pub struct DriveInfo {
    pub name: String,
    pub volume_label: String,
    pub total_space: u64,
    pub available_space: u64,
}

/// Wrapper around FsNode for UI display purposes.
#[derive(Clone, Debug, PartialEq)]
pub struct UiNode {
    pub node: FsNode,
    pub depth: u32,
    pub expanded: bool,
    pub scan_progress: f64,
}

/// Represents a size delta with classification and display color.
#[derive(Clone, Debug, PartialEq)]
pub struct SizeChange {
    pub delta: i64,
    pub classification: String,
    pub hex_color: String,
}

/// Messages emitted during a scan operation.
#[derive(Clone, Debug, PartialEq)]
pub enum ScanMessage {
    DirScanned { path: String, size: u64 },
    ScanError { path: String, error: String },
    Complete { total_size: u64 },
}

Now make sure the workspace knows about this crate. If there is a top-level `Cargo.toml` workspace, it needs to include `crates/models` as a member. Based on the project structure, here's the workspace file (if one already exists, only the `members` list matters — I'm providing a minimal workspace definition):