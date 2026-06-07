//! Entry point for the views desktop application.

use anyhow::Result;
use gpui::{App, AppContext, Application, WindowOptions};
use log::info;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting views application");

    let app = Application::new();

    app.run(|cx: &mut AppContext| {
        let options = WindowOptions {
            titlebar: None,
            window_min_size: Some(gpui::Size {
                width: gpui::px(800.0),
                height: gpui::px(600.0),
            }),
            ..Default::default()
        };

        cx.open_window(options, |cx| {
            cx.new(|_cx| views::AppView::new())
        })
        .expect("failed to open window");
    });

    Ok(())
}