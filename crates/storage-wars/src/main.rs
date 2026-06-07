//! Storage Wars — desktop application entry point.
//!
//! Initializes the GPUI application with a 1280×800 window,
//! dark theme, centered positioning, and a custom title bar.

#![allow(unused)]

mod app;
mod theme;
mod title_bar;
mod ui;

use anyhow::Result;
use gpui::{
    actions, App, Application, Bounds, Context, Menu, MenuItem, Point, Size, TitlebarOptions,
    WindowBounds, WindowKind, WindowOptions,
};
use log::info;

use app::StorageWarsApp;
use theme::StorageWarsTheme;

/// Application identifier used by the OS for window management.
const APP_ID: &str = "com.storage-wars.app";

/// Default window width in logical pixels.
const WINDOW_WIDTH: f32 = 1280.0;

/// Default window height in logical pixels.
const WINDOW_HEIGHT: f32 = 800.0;

/// Application display name shown in menus and the title bar.
const APP_NAME: &str = "Storage Wars";

actions!(storage_wars, [Quit]);

fn main() {
    // Initialize logging — respects the `RUST_LOG` environment variable.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting {} v{}", APP_NAME, env!("CARGO_PKG_VERSION"));

    Application::new().run(|cx: &mut App| {
        // Register global application menu (macOS menu bar / system menu).
        cx.set_menus(build_menus());

        // Bind the Quit action to the standard keyboard shortcut.
        cx.on_action(|_: &Quit, cx| cx.quit());

        // Open the primary application window.
        open_main_window(cx).expect("Failed to open main window");
    });
}

/// Constructs the application menu bar.
fn build_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: APP_NAME.into(),
            items: vec![
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            name: "File".into(),
            items: vec![],
        },
        Menu {
            name: "View".into(),
            items: vec![],
        },
        Menu {
            name: "Help".into(),
            items: vec![],
        },
    ]
}

/// Opens the main application window centered on the primary display.
fn open_main_window(cx: &mut App) -> Result<()> {
    let window_size = Size {
        width: gpui::px(WINDOW_WIDTH),
        height: gpui::px(WINDOW_HEIGHT),
    };

    let window_options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            window_size,
            cx,
        ))),
        titlebar: Some(TitlebarOptions {
            title: Some(APP_NAME.into()),
            appears_transparent: true,
            traffic_light_position: Some(Point {
                x: gpui::px(12.0),
                y: gpui::px(12.0),
            }),
        }),
        window_min_size: Some(Size {
            width: gpui::px(800.0),
            height: gpui::px(600.0),
        }),
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: None,
        window_background: gpui::WindowBackgroundAppearance::Blurred,
        app_id: Some(APP_ID.into()),
        ..Default::default()
    };

    cx.open_window(window_options, |window, cx| {
        cx.new(|cx| StorageWarsApp::new(window, cx))
    })?;

    Ok(())
}