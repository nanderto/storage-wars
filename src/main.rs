use gpui::prelude::*;
use gpui::{App, Bounds, SharedString, WindowBounds, WindowOptions, size};

use storage_wars::app_view::AppView;
use storage_wars::models::DriveInfo;

fn enumerate_drives() -> Vec<DriveInfo> {
    use sysinfo::Disks;
    let disks = Disks::new_with_refreshed_list();
    disks
        .list()
        .iter()
        .map(|d| {
            let mount = d.mount_point().to_string_lossy();
            // On Windows, mount_point() returns "C:\", "D:\", etc.
            // Trim to the drive letter + colon (e.g. "C:").
            let name = if mount.len() >= 2 && mount.as_bytes().get(1) == Some(&b':') {
                mount[..2].to_string()
            } else {
                mount.trim_end_matches(['\\', '/']).to_string()
            };
            DriveInfo {
                name,
                volume_label: d.name().to_string_lossy().into_owned(),
                total_space: d.total_space(),
                available_space: d.available_space(),
            }
        })
        .collect()
}

fn main() {
    let drives = enumerate_drives();

    gpui_platform::application().run(move |cx: &mut App| {
        let drives = drives.clone();
        let bounds = Bounds::centered(None, size(gpui::px(1280.), gpui::px(800.)), cx);
        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some(SharedString::from("Storage Wars")),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| {
                    let mut view = AppView::new(cx);
                    view.set_drives(drives.clone(), cx);
                    view
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
