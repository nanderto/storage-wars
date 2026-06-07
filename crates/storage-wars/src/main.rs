//! Storage Wars — entry point.
//!
//! Initialises the GPUI application, opens a 1280×800 centered window with a
//! dark theme and a custom transparent title bar, then hands control to the
//! GPUI event loop.

use gpui::{
    actions, div, px, size, App, AppContext, Bounds, Context, Element, IntoElement,
    KeyBinding, ParentElement, Point, Render, Styled, TitlebarOptions, ViewContext,
    VisualContext, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
};

mod theme;

// ── Application-level actions ────────────────────────────────────────────────

actions!(storage_wars, [Quit]);

// ── Root view ────────────────────────────────────────────────────────────────

/// The top-level GPUI view for the Storage Wars application.
struct StorageWarsApp;

impl Render for StorageWarsApp {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme::BACKGROUND)
            .text_color(theme::TEXT_PRIMARY)
            // ── Title bar spacer (transparent title bar eats ~28 px on macOS) ──
            .child(div().h(px(28.0)))
            // ── Main content area ─────────────────────────────────────────────
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(theme::ACCENT)
                            .child("Storage Wars"),
                    )
                    .child(
                        div()
                            .text_color(theme::TEXT_SECONDARY)
                            .child("Auction unit storage management"),
                    ),
            )
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    // Initialise logging from the `RUST_LOG` environment variable.
    env_logger::init();

    App::new().run(|cx: &mut AppContext| {
        // Register global quit action.
        cx.on_action(|_: &Quit, cx| cx.quit());

        // Bind ⌘Q (macOS) / Ctrl+Q (Linux / Windows) to Quit.
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("ctrl-q", Quit, None),
        ]);

        // Open the main application window.
        let window_options = build_window_options(cx);
        cx.open_window(window_options, |cx| {
            cx.new_view(|_cx| StorageWarsApp)
        })
        .expect("Failed to open the Storage Wars main window");
    });
}

/// Constructs [`WindowOptions`] for the main 1280×800 centered window.
fn build_window_options(cx: &AppContext) -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(1280.0), px(800.0)),
            cx,
        ))),
        titlebar: Some(TitlebarOptions {
            title: Some("Storage Wars".into()),
            appears_transparent: true,
            traffic_light_position: Some(Point {
                x: px(9.0),
                y: px(9.0),
            }),
        }),
        window_min_size: Some(size(px(800.0), px(600.0))),
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: None,
        window_background: WindowBackgroundAppearance::Blurred,
        focus: true,
        show: true,
        app_id: Some("storage-wars".to_string()),
    }
}