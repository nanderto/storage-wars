use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a storage drive available for scanning
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriveInfo {
    pub id: String,
    pub mount_point: String,
    pub volume_label: Option<String>,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub file_system: Option<String>,
}

impl DriveInfo {
    pub fn used_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.available_bytes)
    }

    pub fn usage_percent(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes() as f64 / self.total_bytes as f64) * 100.0
    }

    /// Format a human-readable label for the drive selector
    pub fn display_label(&self) -> String {
        let size_str = format_bytes(self.total_bytes);
        match &self.volume_label {
            Some(label) if !label.is_empty() => {
                format!("{} ({}) — {}", label, self.mount_point, size_str)
            }
            _ => format!("{} — {}", self.mount_point, size_str),
        }
    }
}

/// A single node in the file system tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub depth: usize,
    pub is_directory: bool,
    pub size_bytes: u64,
    pub file_count: u64,
    pub folder_count: u64,
    pub modified_at: Option<DateTime<Utc>>,
    pub children: Vec<TreeNode>,
    pub is_expanded: bool,
}

impl TreeNode {
    pub fn new_dir(name: impl Into<String>, path: impl Into<String>, depth: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            path: path.into(),
            depth,
            is_directory: true,
            size_bytes: 0,
            file_count: 0,
            folder_count: 0,
            modified_at: None,
            children: Vec::new(),
            is_expanded: false,
        }
    }

    pub fn new_file(
        name: impl Into<String>,
        path: impl Into<String>,
        depth: usize,
        size_bytes: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            path: path.into(),
            depth,
            is_directory: false,
            size_bytes,
            file_count: 1,
            folder_count: 0,
            modified_at: None,
            children: Vec::new(),
            is_expanded: false,
        }
    }
}

/// A completed scan snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSnapshot {
    pub id: Uuid,
    pub drive_id: String,
    pub label: String,
    pub scanned_at: DateTime<Utc>,
    pub total_bytes: u64,
    pub total_files: u64,
    pub total_folders: u64,
    pub root: Option<TreeNode>,
}

impl ScanSnapshot {
    pub fn new(drive_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            drive_id: drive_id.into(),
            label: label.into(),
            scanned_at: Utc::now(),
            total_bytes: 0,
            total_files: 0,
            total_folders: 0,
            root: None,
        }
    }
}

/// Describes how a size has changed between two scans
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeChange {
    /// Size increased significantly
    Increased,
    /// Size decreased significantly
    Decreased,
    /// Size is roughly the same
    Unchanged,
    /// Entry is new (no previous scan)
    New,
    /// Entry was deleted (not in current scan)
    Deleted,
}

impl SizeChange {
    pub fn from_delta(current: u64, previous: u64) -> Self {
        if previous == 0 {
            return SizeChange::New;
        }
        if current == 0 {
            return SizeChange::Deleted;
        }
        let ratio = current as f64 / previous as f64;
        if ratio > 1.05 {
            SizeChange::Increased
        } else if ratio < 0.95 {
            SizeChange::Decreased
        } else {
            SizeChange::Unchanged
        }
    }
}

/// Application-level state shared across views
#[derive(Debug, Clone)]
pub struct AppState {
    pub available_drives: Vec<DriveInfo>,
    pub selected_drive: Option<String>,
    pub scan_history: Vec<ScanSnapshot>,
    pub base_scan_id: Option<Uuid>,
    pub new_scan_id: Option<Uuid>,
    pub is_scanning: bool,
    pub scan_progress: f32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            available_drives: Vec::new(),
            selected_drive: None,
            scan_history: Vec::new(),
            base_scan_id: None,
            new_scan_id: None,
            is_scanning: false,
            scan_progress: 0.0,
        }
    }
}

/// Format bytes into a human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format a percentage value
pub fn format_percent(value: f64) -> String {
    format!("{:.1}%", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1 KB");
        assert_eq!(format_bytes(1536), "2 KB");
        assert_eq!(format_bytes(1_048_576), "1.0 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn test_drive_display_label_with_volume() {
        let drive = DriveInfo {
            id: "C:".to_string(),
            mount_point: "C:".to_string(),
            volume_label: Some("System".to_string()),
            total_bytes: 500 * 1024 * 1024 * 1024,
            available_bytes: 100 * 1024 * 1024 * 1024,
            file_system: Some("NTFS".to_string()),
        };
        let label = drive.display_label();
        assert!(label.contains("System"));
        assert!(label.contains("C:"));
    }

    #[test]
    fn test_drive_display_label_without_volume() {
        let drive = DriveInfo {
            id: "/".to_string(),
            mount_point: "/".to_string(),
            volume_label: None,
            total_bytes: 256 * 1024 * 1024 * 1024,
            available_bytes: 50 * 1024 * 1024 * 1024,
            file_system: Some("ext4".to_string()),
        };
        let label = drive.display_label();
        assert!(label.contains("/"));
        assert!(!label.contains("None"));
    }

    #[test]
    fn test_size_change() {
        assert_eq!(SizeChange::from_delta(0, 100), SizeChange::Deleted);
        assert_eq!(SizeChange::from_delta(100, 0), SizeChange::New);
        assert_eq!(SizeChange::from_delta(200, 100), SizeChange::Increased);
        assert_eq!(SizeChange::from_delta(50, 100), SizeChange::Decreased);
        assert_eq!(SizeChange::from_delta(100, 100), SizeChange::Unchanged);
    }
}