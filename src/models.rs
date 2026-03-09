use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Core filesystem node — used during scanning and tree display
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct FsNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub current_size: u64,
    /// Populated only when comparing two scans.
    pub prev_size: Option<u64>,
    pub children: Vec<FsNode>,
}

// ---------------------------------------------------------------------------
// Flattened node for rendering in the virtual list
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct UiNode {
    pub fs_node: FsNode,
    pub depth: usize,
    pub expanded: bool,
    /// Fraction of the drive/parent consumed: 0.0–1.0. Used to size the bar.
    pub scan_progress: f32,
}

// ---------------------------------------------------------------------------
// Size-change classification
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SizeChange {
    NoBaseline,
    Decreased,
    Unchanged,
    SmallGrowth,  // > 0 – 50 % increase
    MediumGrowth, // 50 – 100 % increase
    LargeGrowth,  // > 100 % increase
}

impl SizeChange {
    pub fn from_node(node: &FsNode) -> Self {
        let Some(prev) = node.prev_size else {
            return SizeChange::NoBaseline;
        };
        let curr = node.current_size;
        if curr < prev {
            SizeChange::Decreased
        } else if curr == prev {
            SizeChange::Unchanged
        } else if prev == 0 {
            // Any growth from zero is treated as large.
            SizeChange::LargeGrowth
        } else {
            let pct = (curr - prev) * 100 / prev;
            if pct <= 50 {
                SizeChange::SmallGrowth
            } else if pct <= 100 {
                SizeChange::MediumGrowth
            } else {
                SizeChange::LargeGrowth
            }
        }
    }

    /// Hex colour string for the status bar overlay.
    pub fn color(self) -> &'static str {
        match self {
            SizeChange::Decreased => "#22c55e",
            SizeChange::Unchanged => "#6b7280",
            SizeChange::SmallGrowth => "#eab308",
            SizeChange::MediumGrowth => "#f97316",
            SizeChange::LargeGrowth => "#ef4444",
            SizeChange::NoBaseline => "#6b7280",
        }
    }
}

// ---------------------------------------------------------------------------
// Scan metadata — one row per scan session
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ScanMeta {
    pub id: i64,
    pub drive: String,
    pub scanned_at: String, // ISO 8601
    pub total_size: u64,
}

// ---------------------------------------------------------------------------
// Database node — flat representation stored in SQLite
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DbNode {
    pub id: i64,
    pub scan_id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
}

// ---------------------------------------------------------------------------
// Drive information supplied by sysinfo
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DriveInfo {
    pub name: String,       // e.g. "C:"
    pub total_space: u64,
    pub available_space: u64,
}

// ---------------------------------------------------------------------------
// Human-readable size formatting
// ---------------------------------------------------------------------------

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1_024;
    const MB: u64 = 1_024 * KB;
    const GB: u64 = 1_024 * MB;
    const TB: u64 = 1_024 * GB;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn node(current_size: u64, prev_size: Option<u64>) -> FsNode {
        FsNode {
            name: "test".into(),
            path: PathBuf::from("/test"),
            is_dir: true,
            current_size,
            prev_size,
            children: vec![],
        }
    }

    // --- SizeChange::from_node ---

    #[test]
    fn size_change_no_baseline() {
        assert_eq!(SizeChange::from_node(&node(100, None)), SizeChange::NoBaseline);
    }

    #[test]
    fn size_change_decreased() {
        assert_eq!(SizeChange::from_node(&node(50, Some(100))), SizeChange::Decreased);
    }

    #[test]
    fn size_change_unchanged() {
        assert_eq!(SizeChange::from_node(&node(100, Some(100))), SizeChange::Unchanged);
    }

    #[test]
    fn size_change_small_growth_boundary_low() {
        // 1 byte growth from 100 → 1% — SmallGrowth
        assert_eq!(SizeChange::from_node(&node(101, Some(100))), SizeChange::SmallGrowth);
    }

    #[test]
    fn size_change_small_growth_boundary_high() {
        // Exactly 50% increase: 150 from 100
        assert_eq!(SizeChange::from_node(&node(150, Some(100))), SizeChange::SmallGrowth);
    }

    #[test]
    fn size_change_medium_growth_boundary_low() {
        // 51% increase: 151 from 100
        assert_eq!(SizeChange::from_node(&node(151, Some(100))), SizeChange::MediumGrowth);
    }

    #[test]
    fn size_change_medium_growth_boundary_high() {
        // Exactly 100% increase: 200 from 100
        assert_eq!(SizeChange::from_node(&node(200, Some(100))), SizeChange::MediumGrowth);
    }

    #[test]
    fn size_change_large_growth() {
        // 201% increase: 201 from 100 — wait, (201-100)*100/100 = 101 → LargeGrowth
        assert_eq!(SizeChange::from_node(&node(201, Some(100))), SizeChange::LargeGrowth);
    }

    #[test]
    fn size_change_zero_prev_size() {
        // Any growth from zero → LargeGrowth
        assert_eq!(SizeChange::from_node(&node(1, Some(0))), SizeChange::LargeGrowth);
    }

    #[test]
    fn size_change_zero_both() {
        // Zero → zero: unchanged
        assert_eq!(SizeChange::from_node(&node(0, Some(0))), SizeChange::Unchanged);
    }

    // --- format_size ---

    #[test]
    fn format_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(1), "1 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn format_kilobytes() {
        assert_eq!(format_size(1_024), "1.00 KB");
        assert_eq!(format_size(1_536), "1.50 KB");
    }

    #[test]
    fn format_megabytes() {
        assert_eq!(format_size(1_048_576), "1.00 MB");
        assert_eq!(format_size(1_572_864), "1.50 MB");
    }

    #[test]
    fn format_gigabytes() {
        assert_eq!(format_size(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn format_terabytes() {
        assert_eq!(format_size(1_099_511_627_776), "1.00 TB");
    }
}
