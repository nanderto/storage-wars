//! Shared data types used across view components.

use serde::{Deserialize, Serialize};

/// Represents a mounted drive or volume.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveInfo {
    /// Mount point path (e.g. `C:\` on Windows, `/dev/sda1` on Linux).
    pub mount_point: String,
    /// Optional human-readable volume label.
    pub volume_label: Option<String>,
    /// Total capacity in bytes.
    pub total_bytes: u64,
    /// Available (free) bytes.
    pub available_bytes: u64,
}

impl DriveInfo {
    /// Returns a formatted label: `"Label (C:\) — 42.3 GB free"` or `"C:\ — 42.3 GB free"`.
    pub fn display_label(&self) -> String {
        let free = format_bytes(self.available_bytes);
        let total = format_bytes(self.total_bytes);
        match &self.volume_label {
            Some(label) if !label.is_empty() => {
                format!("{} ({}) — {} / {} free", label, self.mount_point, free, total)
            }
            _ => format!("{} — {} / {} free", self.mount_point, free, total),
        }
    }
}

/// A single node in the scanned file-system tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TreeNode {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub prev_size_bytes: Option<u64>,
    pub file_count: u64,
    pub folder_count: u64,
    pub modified: Option<String>,
    pub depth: usize,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub children: Vec<TreeNode>,
    /// Fraction of parent size in `[0.0, 1.0]`.
    pub parent_fraction: f32,
}

impl TreeNode {
    /// Percentage change relative to previous scan, if available.
    pub fn size_change_pct(&self) -> Option<f32> {
        let prev = self.prev_size_bytes?;
        if prev == 0 {
            return None;
        }
        Some((self.size_bytes as f32 - prev as f32) / prev as f32 * 100.0)
    }
}

/// A persisted scan record shown in [`ScanHistory`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScanRecord {
    pub id: u64,
    pub drive_mount: String,
    pub label: String,
    pub scanned_at: String,
    pub total_bytes: u64,
}

/// Selection state for the scan history panel.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HistorySelection {
    pub base_id: Option<u64>,
    pub new_id: Option<u64>,
}

/// Visual indicator for size change direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeChange {
    Increased,
    Decreased,
    Unchanged,
}

impl SizeChange {
    pub fn from_pct(pct: Option<f32>) -> Self {
        match pct {
            Some(p) if p > 0.5 => Self::Increased,
            Some(p) if p < -0.5 => Self::Decreased,
            _ => Self::Unchanged,
        }
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// Format a byte count as a human-readable string (GiB / MiB / KiB / B).
pub fn format_bytes(bytes: u64) -> String {
    const GIB: u64 = 1 << 30;
    const MIB: u64 = 1 << 20;
    const KIB: u64 = 1 << 10;

    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_gib() {
        assert_eq!(format_bytes(1 << 30), "1.0 GiB");
    }

    #[test]
    fn format_bytes_mib() {
        assert_eq!(format_bytes(1 << 20), "1.0 MiB");
    }

    #[test]
    fn format_bytes_kib() {
        assert_eq!(format_bytes(1 << 10), "1.0 KiB");
    }

    #[test]
    fn format_bytes_b() {
        assert_eq!(format_bytes(512), "512 B");
    }

    #[test]
    fn drive_info_label_with_volume() {
        let d = DriveInfo {
            mount_point: "C:\\".into(),
            volume_label: Some("System".into()),
            total_bytes: 1 << 30,
            available_bytes: 1 << 29,
        };
        assert!(d.display_label().contains("System"));
        assert!(d.display_label().contains("C:\\"));
    }

    #[test]
    fn drive_info_label_without_volume() {
        let d = DriveInfo {
            mount_point: "/".into(),
            volume_label: None,
            total_bytes: 1 << 30,
            available_bytes: 1 << 29,
        };
        assert!(d.display_label().starts_with('/'));
    }

    #[test]
    fn size_change_from_pct() {
        assert_eq!(SizeChange::from_pct(Some(10.0)), SizeChange::Increased);
        assert_eq!(SizeChange::from_pct(Some(-10.0)), SizeChange::Decreased);
        assert_eq!(SizeChange::from_pct(Some(0.0)), SizeChange::Unchanged);
        assert_eq!(SizeChange::from_pct(None), SizeChange::Unchanged);
    }
}