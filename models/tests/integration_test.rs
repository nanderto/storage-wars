//! Integration tests verifying that all public model types work together.

use models::{
    DbNode, DriveInfo, FsNode, ScanMessage, ScanMeta, SizeChange, SizeChangeTrend, UiNode,
};

#[test]
fn test_fs_node_to_ui_node_conversion() {
    let fs = FsNode::new_dir("home", "/home");
    let ui: UiNode = fs.clone().into();
    assert_eq!(ui.node, fs);
    assert_eq!(ui.depth, 0);
    assert!(!ui.is_expanded);
}

#[test]
fn test_fs_node_to_db_node_fields_match() {
    let fs = FsNode::new_file("report.pdf", "/docs/report.pdf", 204_800);
    let db = DbNode::new(1, Some(0), 1, &fs.name, &fs.path, fs.size, fs.is_dir);
    assert_eq!(db.name, fs.name);
    assert_eq!(db.path, fs.path);
    assert_eq!(db.size, fs.size);
    assert_eq!(db.is_dir, fs.is_dir);
}

#[test]
fn test_size_change_with_fs_node_delta() {
    let mut node = FsNode::new_file("data.bin", "/data.bin", 2_000_000);
    node.prev_size = Some(1_000_000);

    let change = SizeChange::compute(node.size, node.prev_size);
    assert_eq!(change.trend, SizeChangeTrend::LargeIncrease);
    assert_eq!(change.delta_bytes, 1_000_000);
}

#[test]
fn test_scan_message_complete_carries_root() {
    let root = FsNode::new_dir("root", "C:\\");
    let msg = ScanMessage::Complete {
        root: root.clone(),
        total_files: 500,
        total_folders: 25,
        total_size: 10_000_000_000,
        error_count: 0,
    };
    assert!(msg.is_complete());
    assert_eq!(msg.path(), Some("C:\\"));
}

#[test]
fn test_drive_info_used_space_consistency() {
    let drive = DriveInfo::new(
        "D:\\",
        Some("Data".to_string()),
        1_000_000_000_000,
        600_000_000_000,
    );
    assert_eq!(drive.used_space(), 400_000_000_000);
    assert!((drive.used_percent() - 40.0).abs() < 1e-6);
}

#[test]
fn test_scan_meta_lifecycle() {
    let mut meta = ScanMeta::new(1, "/mnt/data");
    assert!(!meta.is_complete);
    meta.total_files = 1_234;
    meta.total_folders = 56;
    meta.total_size = 7_890_000;
    meta.mark_complete();
    assert!(meta.is_complete);
    assert!(meta.duration_secs().is_some());
}

#[test]
fn test_all_size_change_trends_have_colors() {
    let trends = [
        SizeChangeTrend::New,
        SizeChangeTrend::LargeIncrease,
        SizeChangeTrend::SmallIncrease,
        SizeChangeTrend::Unchanged,
        SizeChangeTrend::SmallDecrease,
        SizeChangeTrend::LargeDecrease,
        SizeChangeTrend::Deleted,
    ];
    for trend in trends {
        let color = SizeChange::color_for(trend);
        assert!(
            color.starts_with('#') && color.len() == 7,
            "Invalid color '{color}' for trend {trend:?}"
        );
    }
}

#[test]
fn test_ui_node_scan_progress_workflow() {
    let fs = FsNode::new_dir("src", "/project/src");
    let mut ui = UiNode::new(fs, 1);

    assert!(!ui.is_scanning());
    ui.set_scan_progress(0.0);
    assert!(ui.is_scanning());
    ui.set_scan_progress(0.5);
    assert_eq!(ui.scan_progress, Some(0.5));
    ui.clear_scan_progress();
    assert!(!ui.is_scanning());
}