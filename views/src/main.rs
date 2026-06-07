//! Entry point for the `views` desktop application.

use gpui::{App, Application, WindowOptions};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                titlebar: None,
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| views::app_view::AppView::build(cx)),
        )
        .expect("failed to open main window");
    });
}