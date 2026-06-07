//! Storage Wars — Desktop Application Entry Point
//!
//! Initializes the GPUI application with a 1280×800 window,
//! dark theme, centered positioning, and a custom title bar.

mod theme;
mod title_bar;
mod ui;

use anyhow::Result;
use gpui::{
    actions, div, px, size, App, AppContext, Application, Bounds, Context, Entity, Hsla,
    InteractiveElement, IntoElement, ParentElement, Point, Render, Styled, TitlebarOptions,
    ViewContext, VisualContext, WindowBounds, WindowKind, WindowOptions,
};

use theme::StorageWarsTheme;
use title_bar::TitleBar;

actions!(storage_wars, [Quit]);

/// Width of the main application window in logical pixels.
const WINDOW_WIDTH: f32 = 1280.0;

/// Height of the main application window in logical pixels.
const WINDOW_HEIGHT: f32 = 800.0;

/// Root view that owns the application layout.
struct StorageWarsApp {
    theme: StorageWarsTheme,
}

impl StorageWarsApp {
    fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            theme: StorageWarsTheme::dark(),
        }
    }
}

impl Render for StorageWarsApp {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let bg = self.theme.background;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(bg)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_4()
                    .child(ui::placeholder_content(&self.theme)),
            )
    }
}

fn main() {
    // Initialise logging — respects RUST_LOG env var, defaults to info.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Storage Wars v{}", env!("CARGO_PKG_VERSION"));

    Application::new().run(|cx: &mut AppContext| {
        // Register global quit action.
        cx.on_action(|_: &Quit, cx| cx.quit());

        let window_options = build_window_options(cx);

        cx.open_window(window_options, |cx| {
            cx.new_view(StorageWarsApp::new)
        })
        .expect("failed to open main window");

        cx.activate(true);
    });
}

/// Constructs [`WindowOptions`] for the main 1280×800 centered window.
fn build_window_options(_cx: &mut AppContext) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: Point::default(), // GPUI will center when origin is zero
            size: size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)),
        })),
        titlebar: Some(TitlebarOptions {
            title: Some("Storage Wars".into()),
            appears_transparent: true,
            traffic_light_position: None,
        }),
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: None,
        window_background: gpui::WindowBackgroundAppearance::Blurred,
        app_id: Some("storage-wars".into()),
    }
}