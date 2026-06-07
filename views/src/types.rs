use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a storage drive or mount point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    pub id: Uuid,
    pub path: String,
    pub label: Option<String>,
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub used_bytes: u64,
}

impl DriveInfo {
    pub fn display_label(&self) -> String {
        match &self.label {
            Some(label) if !label.is_empty() => {
                format!("{} ({})", label, self.path)
            }
            _ => self.path.clone(),
        }
    }

    pub fn used_percent(&self) -> f32 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f32 / self.total_bytes as f32) * 100.0
    }
}

/// Represents a single scan result entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanEntry {
    pub id: Uuid,
    pub drive_id: Uuid,
    pub label: String,
    pub scanned_at: DateTime<Utc>,
    pub total_bytes: u64,
    pub file_count: u64,
    pub folder_count: u64,
}

/// A node in the file-system tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub depth: usize,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub prev_size_bytes: Option<u64>,
    pub file_count: u64,
    pub folder_count: u64,
    pub modified_at: Option<DateTime<Utc>>,
    pub children: Vec<TreeNode>,
    pub is_expanded: bool,
}

impl TreeNode {
    pub fn size_change(&self) -> SizeChange {
        match self.prev_size_bytes {
            None => SizeChange::New,
            Some(prev) if self.size_bytes > prev => SizeChange::Increased,
            Some(prev) if self.size_bytes < prev => SizeChange::Decreased,
            _ => SizeChange::Unchanged,
        }
    }

    pub fn percent_of_parent(&self, parent_size: u64) -> f32 {
        if parent_size == 0 {
            return 0.0;
        }
        (self.size_bytes as f32 / parent_size as f32) * 100.0
    }

    pub fn percent_change(&self) -> Option<f32> {
        self.prev_size_bytes.map(|prev| {
            if prev == 0 {
                return 100.0;
            }
            ((self.size_bytes as f64 - prev as f64) / prev as f64 * 100.0) as f32
        })
    }
}

/// Indicates how a node's size changed relative to a previous scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeChange {
    New,
    Increased,
    Decreased,
    Unchanged,
}

/// Which scan is selected as Base vs New for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanRole {
    Base,
    New,
}

/// Application-level state shared across views.
#[derive(Debug, Clone)]
pub struct AppState {
    pub drives: Vec<DriveInfo>,
    pub selected_drive: Option<Uuid>,
    pub scan_history: Vec<ScanEntry>,
    pub base_scan_id: Option<Uuid>,
    pub new_scan_id: Option<Uuid>,
    pub tree_root: Option<TreeNode>,
    pub is_scanning: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            drives: Vec::new(),
            selected_drive: None,
            scan_history: Vec::new(),
            base_scan_id: None,
            new_scan_id: None,
            tree_root: None,
            is_scanning: false,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mock_data() -> Self {
        use chrono::TimeZone;

        let drive_id = Uuid::new_v4();
        let drives = vec![
            DriveInfo {
                id: drive_id,
                path: "/".to_string(),
                label: Some("Macintosh HD".to_string()),
                total_bytes: 500_000_000_000,
                free_bytes: 120_000_000_000,
                used_bytes: 380_000_000_000,
            },
            DriveInfo {
                id: Uuid::new_v4(),
                path: "/Volumes/Data".to_string(),
                label: None,
                total_bytes: 2_000_000_000_000,
                free_bytes: 800_000_000_000,
                used_bytes: 1_200_000_000_000,
            },
        ];

        let scan1 = ScanEntry {
            id: Uuid::new_v4(),
            drive_id,
            label: "Scan 2024-01-15".to_string(),
            scanned_at: Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap(),
            total_bytes: 370_000_000_000,
            file_count: 142_000,
            folder_count: 8_500,
        };
        let scan2 = ScanEntry {
            id: Uuid::new_v4(),
            drive_id,
            label: "Scan 2024-06-01".to_string(),
            scanned_at: Utc.with_ymd_and_hms(2024, 6, 1, 14, 30, 0).unwrap(),
            total_bytes: 380_000_000_000,
            file_count: 148_000,
            folder_count: 8_900,
        };

        let base_id = scan1.id;
        let new_id = scan2.id;

        Self {
            drives,
            selected_drive: Some(drive_id),
            scan_history: vec![scan1, scan2],
            base_scan_id: Some(base_id),
            new_scan_id: Some(new_id),
            tree_root: Some(mock_tree_root()),
            is_scanning: false,
        }
    }
}

fn mock_tree_root() -> TreeNode {
    TreeNode {
        id: Uuid::new_v4(),
        name: "/".to_string(),
        path: "/".to_string(),
        depth: 0,
        is_dir: true,
        size_bytes: 380_000_000_000,
        prev_size_bytes: Some(370_000_000_000),
        file_count: 148_000,
        folder_count: 8_900,
        modified_at: None,
        is_expanded: true,
        children: vec![
            TreeNode {
                id: Uuid::new_v4(),
                name: "Users".to_string(),
                path: "/Users".to_string(),
                depth: 1,
                is_dir: true,
                size_bytes: 200_000_000_000,
                prev_size_bytes: Some(190_000_000_000),
                file_count: 100_000,
                folder_count: 5_000,
                modified_at: None,
                is_expanded: false,
                children: vec![],
            },
            TreeNode {
                id: Uuid::new_v4(),
                name: "Applications".to_string(),
                path: "/Applications".to_string(),
                depth: 1,
                is_dir: true,
                size_bytes: 80_000_000_000,
                prev_size_bytes: Some(80_000_000_000),
                file_count: 20_000,
                folder_count: 2_000,
                modified_at: None,
                is_expanded: false,
                children: vec![],
            },
            TreeNode {
                id: Uuid::new_v4(),
                name: "System".to_string(),
                path: "/System".to_string(),
                depth: 1,
                is_dir: true,
                size_bytes: 60_000_000_000,
                prev_size_bytes: Some(62_000_000_000),
                file_count: 25_000,
                folder_count: 1_800,
                modified_at: None,
                is_expanded: false,
                children: vec![],
            },
            TreeNode {
                id: Uuid::new_v4(),
                name: "Library".to_string(),
                path: "/Library".to_string(),
                depth: 1,
                is_dir: true,
                size_bytes: 40_000_000_000,
                prev_size_bytes: Some(38_000_000_000),
                file_count: 3_000,
                folder_count: 100,
                modified_at: None,
                is_expanded: false,
                children: vec![],
            },
        ],
    }
}