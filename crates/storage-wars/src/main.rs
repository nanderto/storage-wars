mod app;
mod theme;
mod title_bar;
mod ui;

use anyhow::Result;
use gpui::{App, AppContext, Bounds, Point, Size, WindowBounds, WindowOptions, pixels};
use log::info;

use app::StorageWarsApp;
use theme::StorageWarsTheme;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Storage Wars desktop application");

    App::new().run(|cx: &mut AppContext| {
        let theme = StorageWarsTheme::dark();
        cx.set_global(theme);

        let window_options = build_window_options(cx);

        cx.open_window(window_options, |cx| {
            cx.new_view(|cx| StorageWarsApp::new(cx))
        })
        .expect("Failed to open Storage Wars window");
    });

    Ok(())
}

fn build_window_options(cx: &AppContext) -> WindowOptions {
    const WINDOW_WIDTH: f32 = 1280.0;
    const WINDOW_HEIGHT: f32 = 800.0;

    let window_size = Size {
        width: pixels(WINDOW_WIDTH),
        height: pixels(WINDOW_HEIGHT),
    };

    // Attempt to center the window on the primary display.
    let center_position = cx
        .primary_display()
        .and_then(|display| {
            let display_bounds = display.bounds();
            let x = display_bounds.origin.x
                + (display_bounds.size.width - pixels(WINDOW_WIDTH)) / 2.0;
            let y = display_bounds.origin.y
                + (display_bounds.size.height - pixels(WINDOW_HEIGHT)) / 2.0;
            Some(Point { x, y })
        })
        .unwrap_or(Point {
            x: pixels(0.0),
            y: pixels(0.0),
        });

    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: center_position,
            size: window_size,
        })),
        titlebar: Some(gpui::TitlebarOptions {
            title: Some("Storage Wars".into()),
            appears_transparent: true,
            traffic_light_position: Some(Point {
                x: pixels(12.0),
                y: pixels(12.0),
            }),
        }),
        focus: true,
        show: true,
        kind: gpui::WindowKind::Normal,
        is_movable: true,
        display_id: None,
        window_background: gpui::WindowBackgroundAppearance::Opaque,
        app_id: Some("storage-wars".to_string()),
    }
}