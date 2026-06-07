mod app_view;
mod drive_selector;
mod scan_history;
mod title_bar;
mod tree_view;
mod theme;
mod types;

use anyhow::Result;
use gpui::{
    actions, App, Application, Bounds, Context, TitlebarOptions, Window, WindowBounds,
    WindowOptions,
};
use log::info;

actions!(views_app, [Quit]);

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting views application");

    let app = Application::new();

    app.run(|cx: &mut App| {
        cx.on_action(|_: &Quit, cx| cx.quit());

        cx.bind_keys([gpui::KeyBinding::new(
            "cmd-q",
            Quit,
            None,
        )]);

        let bounds = Bounds::centered(None, gpui::size(gpui::px(1280.0), gpui::px(800.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Disk Analyzer — Views".into()),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                focus: true,
                show: true,
                kind: gpui::WindowKind::Normal,
                is_movable: true,
                display_id: None,
                window_background: gpui::WindowBackgroundAppearance::Opaque,
                app_id: Some("com.diskanalyzer.views".to_string()),
                window_min_size: Some(gpui::size(gpui::px(800.0), gpui::px(600.0))),
                window_decorations: None,
            },
            |window, cx| {
                cx.new(|cx| app_view::AppView::new(window, cx))
            },
        )
        .expect("failed to open window");
    });

    Ok(())
}