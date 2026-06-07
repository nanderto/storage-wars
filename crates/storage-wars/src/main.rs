mod theme;
mod title_bar;

use anyhow::Result;
use gpui::{
    actions, div, px, size, App, Application, Bounds, Context, Element, Entity, Render,
    TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions,
};
use theme::StorageWarsTheme;
use title_bar::TitleBar;

actions!(storage_wars, [Quit]);

/// Root application view — owns the window layout and orchestrates sub-modules.
struct StorageWarsApp {
    theme: StorageWarsTheme,
}

impl StorageWarsApp {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            theme: StorageWarsTheme::dark(),
        }
    }
}

impl Render for StorageWarsApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl Element {
        let bg = self.theme.background;
        let fg = self.theme.foreground;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(bg)
            .text_color(fg)
            .child(TitleBar::render(&self.theme, cx))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Storage Wars"),
                    ),
            )
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Storage Wars");

    Application::new().run(|cx: &mut App| {
        // Register global quit action
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([gpui::KeyBinding::new("cmd-q", Quit, None)]);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: gpui::Point {
                    x: px(0.0),
                    y: px(0.0),
                },
                size: size(px(1280.0), px(800.0)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Storage Wars".into()),
                appears_transparent: true,
                traffic_light_position: Some(gpui::Point {
                    x: px(12.0),
                    y: px(12.0),
                }),
            }),
            window_min_size: Some(size(px(800.0), px(600.0))),
            kind: WindowKind::Normal,
            is_movable: true,
            display_id: None,
            window_background: gpui::WindowBackgroundAppearance::Blurred,
            focus: true,
            show: true,
            ..Default::default()
        };

        cx.open_window(window_options, |_window, cx| {
            cx.new(|cx| StorageWarsApp::new(cx))
        })
        .expect("Failed to open main window");
    });

    Ok(())
}