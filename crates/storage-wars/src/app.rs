use gpui::{
    div, px, relative, AnyView, Div, Element, EventEmitter, FocusHandle, FocusableView,
    InteractiveElement, IntoElement, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WindowContext,
};

use crate::theme::StorageWarsTheme;
use crate::title_bar::TitleBar;
use crate::ui::MainContent;

/// Root application view. Owns the window layout and orchestrates all sub-views.
pub struct StorageWarsApp {
    focus_handle: FocusHandle,
    title_bar: View<TitleBar>,
    main_content: View<MainContent>,
}

impl StorageWarsApp {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let title_bar = cx.new_view(|cx| TitleBar::new(cx));
        let main_content = cx.new_view(|cx| MainContent::new(cx));

        Self {
            focus_handle,
            title_bar,
            main_content,
        }
    }
}

impl FocusableView for StorageWarsApp {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for StorageWarsApp {}

impl Render for StorageWarsApp {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<StorageWarsTheme>();
        let bg = theme.background;
        let fg = theme.foreground;

        div()
            .key_context("StorageWars")
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .bg(bg)
            .text_color(fg)
            .child(self.title_bar.clone())
            .child(self.main_content.clone())
    }
}