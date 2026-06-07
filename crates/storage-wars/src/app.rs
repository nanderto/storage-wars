//! Root application view — the top-level GPUI element rendered inside the
//! main window.

use gpui::{
    div, px, App, Context, Element, Entity, Focusable, FocusHandle, IntoElement, ParentElement,
    Render, Styled, Window,
};

use crate::theme::StorageWarsTheme;
use crate::title_bar::TitleBar;
use crate::ui::content::ContentArea;

/// The root view of the Storage Wars application.
///
/// Owns the focus handle for the window and composes the title bar with the
/// main content area.
pub struct StorageWarsApp {
    focus_handle: FocusHandle,
    theme: StorageWarsTheme,
}

impl StorageWarsApp {
    /// Creates a new [`StorageWarsApp`] instance.
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            focus_handle,
            theme: StorageWarsTheme::dark(),
        }
    }
}

impl Focusable for StorageWarsApp {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StorageWarsApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;

        div()
            .key_context("StorageWars")
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .font_family("system-ui")
            .child(TitleBar::new(theme.clone()))
            .child(ContentArea::new(theme.clone()))
    }
}