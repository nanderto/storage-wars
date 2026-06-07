use gpui::*;

struct StorageWarsApp;

impl Render for StorageWarsApp {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .flex()
            .items_center()
            .justify_center()
            .child("Storage Wars")
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(
            None,
            size(px(1280.0), px(800.0)),
            cx,
        );

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Storage Wars".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| StorageWarsApp),
        )
        .expect("failed to open window");
    });
}