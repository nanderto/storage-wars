//! Entry point for the views desktop application.

use anyhow::Result;
use gpui::{App, Application};
use views::AppView;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    Application::new().run(|cx: &mut App| {
        AppView::open(cx);
    });

    Ok(())
}