//! [`AppView`] — root orchestrator view.

use gpui::*;

use crate::drive_selector::{DriveSelected, DriveSelector};
use crate::scan_history::ScanHistory;
use crate::theme::{self, TITLE_BAR_HEIGHT_PX};
use crate::tree_view::TreeView;
use crate::types::{Drive, ScanState};

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// Root application view: title bar, drive selector, scan controls,
/// tree view, scan history panel, and drive info panel.
pub struct AppView {
    drive_selector: View<DriveSelector>,
    tree_view: View<TreeView>,
    scan_history: View<ScanHistory>,
    scan_state: ScanState,
    selected_drive: Option<Drive>,
    _subscriptions: Vec<Subscription>,
}

impl AppView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let drive_selector = cx.new_view(DriveSelector::new);
        let tree_view = cx.new_view(TreeView::new);
        let scan_history = cx.new_view(ScanHistory::new);

        // Subscribe to drive selection events.
        let sub = cx.subscribe(&drive_selector, |this, _view, event: &DriveSelected, cx| {
            this.selected_drive = Some(event.0.clone());
            cx.notify();
        });

        // Seed with example drives for demonstration.
        drive_selector.update(cx, |ds, cx| {
            ds.set_drives(example_drives(), cx);
        });

        Self {
            drive_selector,
            tree_view,
            scan_history,
            scan_state: ScanState::Idle,
            selected_drive: None,
            _subscriptions: vec![sub],
        }
    }

    fn start_scan(&mut self, cx: &mut ViewContext<Self>) {
        if self.selected_drive.is_none() {
            return;
        }
        self.scan_state = ScanState::Scanning {
            progress: 0.0,
            current_path: String::new(),
        };
        cx.notify();
        // In a real implementation, spawn an async task here.
    }

    // -----------------------------------------------------------------------
    // Rendering helpers
    // -----------------------------------------------------------------------

    fn render_title_bar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("title-bar")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(TITLE_BAR_HEIGHT_PX))
            .bg(theme::title_bar_bg())
            .border_b_1()
            .border_color(theme::border())
            // Make the title bar draggable.
            .on_mouse_down(MouseButton::Left, |_ev, cx| cx.start_window_move())
            // App title
            .child(
                div()
                    .flex()
                    .items_center()
                    .pl(px(12.0))
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_color(theme::text_primary())
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("Disk Space Analyzer"),
                    ),
            )
            // Window controls (close / minimise / maximise)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(0.0))
                    .pr(px(4.0))
                    .child(self.window_control_button("─", cx, |cx| cx.minimize_window()))
                    .child(self.window_control_button("□", cx, |cx| cx.zoom_window()))
                    .child(self.window_control_button("✕", cx, |cx| cx.remove_window())),
            )
    }

    fn window_control_button(
        &self,
        label: &'static str,
        cx: &mut ViewContext<Self>,
        action: impl Fn(&mut WindowContext) + 'static,
    ) -> impl IntoElement {
        div()
            .id(ElementId::Name(format!("wc-{label}").into()))
            .w(px(46.0))
            .h(px(TITLE_BAR_HEIGHT_PX))
            .flex()
            .items_center()
            .justify_center()
            .text_color(theme::text_secondary())
            .text_sm()
            .cursor_pointer()
            .hover(|s| s.bg(theme::bg_hover()))
            .on_click(move |_ev, cx| action(cx))
            .child(label)
    }

    fn render_toolbar(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_scanning = matches!(self.scan_state, ScanState::Scanning { .. });
        let scan_label: &'static str = if is_scanning { "Scanning…" } else { "Scan" };

        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(48.0))
            .px(px(12.0))
            .gap(px(8.0))
            .bg(theme::bg_secondary())
            .border_b_1()
            .border_color(theme::border())
            // Drive selector takes remaining space.
            .child(div().flex_grow().child(self.drive_selector.clone()))
            // Scan button
            .child(
                div()
                    .id("scan-btn")
                    .px(px(16.0))
                    .h(px(32.0))
                    .flex()
                    .items_center()
                    .rounded(px(4.0))
                    .bg(if is_scanning {
                        theme::bg_secondary()
                    } else {
                        theme::accent()
                    })
                    .text_color(if is_scanning {
                        theme::text_secondary()
                    } else {
                        theme::bg_primary()
                    })
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .cursor_pointer()
                    .when(!is_scanning, |el| {
                        el.on_click(cx.listener(|this, _ev, cx| this.start_scan(cx)))
                    })
                    .child(scan_label),
            )
    }

    fn render_status_bar(&self) -> impl IntoElement {
        let status: SharedString = match &self.scan_state {
            ScanState::Idle => "Ready".into(),
            ScanState::Scanning { current_path, .. } => {
                format!("Scanning: {current_path}").into()
            }
            ScanState::Complete => "Scan complete".into(),
            ScanState::Error(e) => format!("Error: {e}").into(),
        };

        div()
            .flex()
            .items_center()
            .w_full()
            .h(px(24.0))
            .px(px(12.0))
            .bg(theme::bg_secondary())
            .border_t_1()
            .border_color(theme::border())
            .child(
                div()
                    .text_color(theme::text_secondary())
                    .text_xs()
                    .child(status),
            )
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for AppView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("app-view")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme::bg_primary())
            // Title bar
            .child(self.render_title_bar(cx))
            // Toolbar (drive selector + scan button)
            .child(self.render_toolbar(cx))
            // Main content area
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_grow()
                    .overflow_hidden()
                    // Scan history panel (left)
                    .child(self.scan_history.clone())
                    // Tree view (centre, fills remaining space)
                    .child(
                        div()
                            .flex_grow()
                            .h_full()
                            .overflow_hidden()
                            .child(self.tree_view.clone()),
                    ),
            )
            // Status bar
            .child(self.render_status_bar())
    }
}

// ---------------------------------------------------------------------------
// Example data
// ---------------------------------------------------------------------------

fn example_drives() -> Vec<Drive> {
    vec![
        Drive {
            id: "C:".to_string(),
            path: "C:\\".to_string(),
            volume_label: Some("System".to_string()),
            total_bytes: 512 * 1024 * 1024 * 1024,
            available_bytes: 128 * 1024 * 1024 * 1024,
        },
        Drive {
            id: "D:".to_string(),
            path: "D:\\".to_string(),
            volume_label: Some("Data".to_string()),
            total_bytes: 2 * 1024 * 1024 * 1024 * 1024,
            available_bytes: 900 * 1024 * 1024 * 1024,
        },
    ]
}