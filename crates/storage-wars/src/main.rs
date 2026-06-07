mod app;
mod theme;
mod title_bar;
mod ui;

use anyhow::Result;
use gpui::{Application, WindowOptions};
use log::info;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Storage Wars v{}", env!("CARGO_PKG_VERSION"));

    let app = Application::new();

    app.run(|cx| {
        // Apply dark theme
        theme::apply_dark_theme(cx);

        // Open the main application window
        let window_options = app::main_window_options(cx);

        cx.open_window(window_options, |cx| {
            cx.new(|cx| app::StorageWarsApp::new(cx))
        })
        .expect("Failed to open main window");
    });

    Ok(())
}