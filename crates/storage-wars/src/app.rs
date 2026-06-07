use gpui::{
    div, px, size, App, AppContext, Bounds, Context, Entity, IntoElement, ParentElement, Point,
    Render, Styled, Window, WindowBounds, WindowKind, WindowOptions,
};

use crate::title_bar::TitleBar;
use crate::ui::MainView;

/// Width of the main application window in logical pixels.
pub const WINDOW_WIDTH: f32 = 1280.0;

/// Height of the main application window in logical pixels.
pub const WINDOW_HEIGHT: f32 = 800.0;

/// Root application model that owns the top-level window state.
pub struct StorageWarsApp {
    title_bar: Entity<TitleBar>,
    main_view: Entity<MainView>,
}

impl StorageWarsApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let title_bar = cx.new(|cx| TitleBar::new(cx));
        let main_view = cx.new(|cx| MainView::new(cx));

        Self {
            title_bar,
            main_view,
        }
    }
}

impl Render for StorageWarsApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::rgb(0x1a1a2e))
            .text_color(gpui::rgb(0xe0e0e0))
            .child(self.title_bar.clone())
            .child(self.main_view.clone())
    }
}

/// Build the [`WindowOptions`] for the main Storage Wars window.
///
/// The window is 1280×800, centered on the primary display, and uses a
/// custom title bar so the OS chrome is hidden.
pub fn main_window_options(cx: &App) -> WindowOptions {
    // Attempt to center the window on the primary display.
    let bounds = cx
        .primary_display()
        .map(|display| {
            let display_bounds = display.bounds();
            let x = display_bounds.origin.x
                + (display_bounds.size.width - px(WINDOW_WIDTH)) / 2.0;
            let y = display_bounds.origin.y
                + (display_bounds.size.height - px(WINDOW_HEIGHT)) / 2.0;

            WindowBounds::Windowed(Bounds {
                origin: Point { x, y },
                size: size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)),
            })
        })
        .unwrap_or_else(|| {
            WindowBounds::Windowed(Bounds {
                origin: Point::default(),
                size: size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)),
            })
        });

    WindowOptions {
        window_bounds: Some(bounds),
        titlebar: None, // Custom title bar — suppress OS chrome
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: cx.primary_display().map(|d| d.id()),
        window_background: gpui::WindowBackgroundAppearance::Opaque,
        app_id: Some("storage-wars".to_string()),
    }
}