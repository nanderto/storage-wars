use gpui::prelude::*;
use gpui::{App, Bounds, WindowBounds, WindowOptions, size};
use gpui_component::TitleBar;
use gpui_component::theme::{Theme, ThemeMode};

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
        gpui_component::init(cx);
        Theme::change(ThemeMode::Dark, None, cx);

        let drives = drives.clone();
        let bounds = Bounds::centered(None, size(gpui::px(1280.), gpui::px(800.)), cx);
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitleBar::title_bar_options()),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| {
                    let mut app_view = AppView::new(window, cx);
                    app_view.set_drives(drives.clone(), window, cx);
                    app_view
                });
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
