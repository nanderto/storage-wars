mod theme;
mod title_bar;

use anyhow::Result;
use gpui::{
    actions, div, px, size, App, AppContext, Application, Bounds, Context, Entity, IntoElement,
    ParentElement, Point, Render, SharedString, Styled, TitlebarOptions, ViewContext,
    VisualContext, WindowBounds, WindowKind, WindowOptions,
};
use theme::StorageWarsTheme;
use title_bar::TitleBar;

actions!(storage_wars, [Quit]);

// ── Root application view ─────────────────────────────────────────────────────

struct StorageWarsApp {
    title_bar: Entity<TitleBar>,
    theme: StorageWarsTheme,
}

impl StorageWarsApp {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        let theme = StorageWarsTheme::dark();
        let title_bar = cx.new(|_cx| TitleBar::new("Storage Wars"));
        Self { title_bar, theme }
    }
}

impl Render for StorageWarsApp {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = self.theme.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            // Custom title bar
            .child(self.title_bar.clone())
            // Main content area
            .child(
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_xl()
                            .text_color(theme.foreground)
                            .child(SharedString::from("Storage Wars")),
                    ),
            )
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Storage Wars desktop application");

    Application::new().run(|cx: &mut AppContext| {
        // Global quit action bound to Cmd-Q / Ctrl-Q.
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([gpui::KeyBinding::new("cmd-q", Quit, None)]);

        let window_options = build_window_options();

        cx.open_window(window_options, |cx| cx.new(StorageWarsApp::new))
            .expect("Failed to open the Storage Wars window");
    });

    Ok(())
}

/// Construct [`WindowOptions`] for a 1280 × 800 centred window with a
/// transparent, custom title bar.
fn build_window_options() -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: Point {
                x: px(0.0),
                y: px(0.0),
            },
            size: size(px(1280.0), px(800.0)),
        })),
        titlebar: Some(TitlebarOptions {
            title: Some(SharedString::from("Storage Wars")),
            appears_transparent: true,
            traffic_light_position: Some(Point {
                x: px(12.0),
                y: px(12.0),
            }),
        }),
        center: true,
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: None,
        window_background: gpui::WindowBackgroundAppearance::Opaque,
        app_id: Some("storage-wars".to_string()),
        window_min_size: Some(size(px(800.0), px(600.0))),
        window_decorations: None,
    }
}