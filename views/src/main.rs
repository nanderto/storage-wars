//! Entry point for the views desktop application.

use anyhow::Result;
use gpui::*;
use log::info;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting views application");

    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Disk Space Analyzer")),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            },
            |cx| cx.new_view(|cx| views::AppView::new(cx)),
        )
        .expect("Failed to open window");
    });

    Ok(())
}