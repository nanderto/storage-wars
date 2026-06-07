mod app_view;
mod drive_selector;
mod scan_history;
mod title_bar;
mod tree_view;
mod types;
mod ui_helpers;

use anyhow::Result;
use gpui::{
    actions, App, Application, AppContext, Bounds, KeyBinding, Menu, MenuItem, TitlebarOptions,
    WindowBounds, WindowKind, WindowOptions,
};
use log::info;

use app_view::AppView;

actions!(views_app, [Quit]);

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting views application");

    Application::new().run(|cx: &mut AppContext| {
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });

        cx.set_menus(vec![Menu {
            name: "Views".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);

        let window_options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: None,
                appears_transparent: true,
                traffic_light_position: None,
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                gpui::size(gpui::px(1200.0), gpui::px(800.0)),
                cx,
            ))),
            kind: WindowKind::Normal,
            is_movable: true,
            display_id: None,
            window_background: gpui::WindowBackgroundAppearance::Blurred,
            focus: true,
            show: true,
            window_min_size: Some(gpui::size(gpui::px(800.0), gpui::px(600.0))),
            app_id: Some("views".to_string()),
        };

        cx.open_window(window_options, |cx| cx.new_view(|cx| AppView::new(cx)))
            .expect("Failed to open main window");
    });

    Ok(())
}