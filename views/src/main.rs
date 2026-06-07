//! Entry point for the views desktop application.

use anyhow::Result;
use gpui::{
    actions, div, px, size, App, AppContext, Application, Bounds, Context, Entity, Render,
    ViewContext, VisualContext, WindowBounds, WindowOptions,
};
use views::AppView;

actions!(views_app, [Quit]);

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting views application");

    Application::new().run(|cx: &mut AppContext| {
        cx.on_action(|_: &Quit, cx| cx.quit());

        cx.bind_keys([gpui::KeyBinding::new(
            "cmd-q",
            Quit,
            None,
        )]);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: gpui::Point {
                    x: px(100.0),
                    y: px(100.0),
                },
                size: size(px(1200.0), px(800.0)),
            })),
            titlebar: Some(gpui::TitlebarOptions {
                title: Some(gpui::SharedString::from("Disk Space Analyzer")),
                appears_transparent: true,
                traffic_light_position: Some(gpui::Point {
                    x: px(12.0),
                    y: px(12.0),
                }),
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |cx| cx.new(|cx| RootView::new(cx)))
            .expect("Failed to open window");
    });

    Ok(())
}

/// Root view that hosts the [`AppView`].
struct RootView {
    app_view: Entity<AppView>,
}

impl RootView {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            app_view: cx.new(|cx| AppView::new(cx)),
        }
    }
}

impl Render for RootView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl gpui::IntoElement {
        div()
            .size_full()
            .child(self.app_view.clone())
    }
}