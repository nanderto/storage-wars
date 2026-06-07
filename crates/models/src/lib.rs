/// Metadata for a single scan session.
#[derive(Clone, Debug, PartialEq)]
pub struct ScanMeta {
    pub id: i64,
    pub drive: String,
    pub timestamp: String,
    pub total_size: u64,
}

/// A node in the in-memory filesystem tree.
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
    pub file_count: u64,
    pub folder_count: u64,
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

/// Wrapper around `FsNode` carrying UI-specific state.
#[derive(Clone, Debug, PartialEq)]
pub struct UiNode {
    pub node: FsNode,
    pub depth: u32,
    pub expanded: bool,
    pub scan_progress: f32,
}

/// Represents a size delta with a classification and display color.
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
    Complete { scan_id: i64 },
}