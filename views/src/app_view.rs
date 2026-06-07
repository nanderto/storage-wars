//! Root application view — orchestrates all child components.

use gpui::{
    div, px, App, AppContext, Div, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window, WindowOptions,
};

use crate::{
    drive_selector::DriveSelector,
    scan_history::ScanHistory,
    theme,
    tree_view::TreeView,
    types::{DriveInfo, HistorySelection, ScanRecord, TreeNode},
};

/// Root view — owns all child components and application state.
pub struct AppView {
    focus_handle: FocusHandle,
    drive_selector: Entity<DriveSelector>,
    tree_view: Entity<TreeView>,
    scan_history: Entity<ScanHistory>,
    drives: Vec<DriveInfo>,
    selected_drive_idx: Option<usize>,
    is_scanning: bool,
}

impl AppView {
    /// Open the main application window.
    pub fn open(cx: &mut App) {
        let options = WindowOptions {
            titlebar: None,
            ..Default::default()
        };

        cx.open_window(options, |_window, cx| {
            cx.new(|cx| Self::new(cx))
        })
        .expect("failed to open main window");
    }

    fn new(cx: &mut gpui::Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        // Stub data so the scaffold compiles and renders.
        let drives = vec![
            DriveInfo {
                mount_point: "C:\\".into(),
                volume_label: Some("System".into()),
                total_bytes: 512 * (1 << 30),
                available_bytes: 128 * (1 << 30),
            },
            DriveInfo {
                mount_point: "D:\\".into(),
                volume_label: Some("Data".into()),
                total_bytes: 1024 * (1 << 30),
                available_bytes: 600 * (1 << 30),
            },
        ];

        let drive_selector = cx.new(|cx| DriveSelector::new(drives.clone(), cx));
        let tree_view = cx.new(|cx| TreeView::new(vec![], cx));
        let scan_history = cx.new(|cx| {
            ScanHistory::new(
                vec![
                    ScanRecord {
                        id: 1,
                        drive_mount: "C:\\".into(),
                        label: "Scan 2024-01-01".into(),
                        scanned_at: "2024-01-01 10:00".into(),
                        total_bytes: 400 * (1 << 30),
                    },
                    ScanRecord {
                        id: 2,
                        drive_mount: "C:\\".into(),
                        label: "Scan 2024-06-15".into(),
                        scanned_at: "2024-06-15 14:30".into(),
                        total_bytes: 384 * (1 << 30),
                    },
                ],
                cx,
            )
        });

        Self {
            focus_handle,
            drive_selector,
            tree_view,
            scan_history,
            drives,
            selected_drive_idx: None,
            is_scanning: false,
        }
    }

    fn render_title_bar(&self, cx: &gpui::Context<Self>) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(theme::TITLE_BAR_HEIGHT))
            .bg(theme::COLOR_TITLE_BAR_BG)
            .px(px(theme::SPACING_MD))
            .child(
                div()
                    .text_color(theme::COLOR_TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child("Disk Space Analyzer"),
            )
            .child(self.render_window_controls(cx))
    }

    fn render_window_controls(&self, _cx: &gpui::Context<Self>) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(theme::SPACING_SM))
            .child(
                div()
                    .w(px(theme::WINDOW_CONTROL_SIZE))
                    .h(px(theme::WINDOW_CONTROL_SIZE))
                    .rounded_full()
                    .bg(theme::COLOR_WINDOW_CONTROL_MINIMIZE),
            )
            .child(
                div()
                    .w(px(theme::WINDOW_CONTROL_SIZE))
                    .h(px(theme::WINDOW_CONTROL_SIZE))
                    .rounded_full()
                    .bg(theme::COLOR_WINDOW_CONTROL_MAXIMIZE),
            )
            .child(
                div()
                    .w(px(theme::WINDOW_CONTROL_SIZE))
                    .h(px(theme::WINDOW_CONTROL_SIZE))
                    .rounded_full()
                    .bg(theme::COLOR_WINDOW_CONTROL_CLOSE),
            )
    }

    fn render_toolbar(&self, cx: &gpui::Context<Self>) -> Div {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(theme::SPACING_SM))
            .w_full()
            .h(px(theme::DRIVE_SELECTOR_HEIGHT + theme::SPACING_MD * 2.0))
            .bg(theme::COLOR_SURFACE)
            .border_b_1()
            .border_color(theme::COLOR_BORDER)
            .px(px(theme::SPACING_MD))
            .child(self.drive_selector.clone())
            .child(self.render_scan_button(cx))
    }

    fn render_scan_button(&self, _cx: &gpui::Context<Self>) -> Div {
        let label = if self.is_scanning { "Scanning…" } else { "Scan" };
        div()
            .flex()
            .items_center()
            .justify_center()
            .h(px(theme::DRIVE_SELECTOR_HEIGHT))
            .px(px(theme::SPACING_LG))
            .rounded(px(4.0))
            .bg(theme::COLOR_BUTTON_BG)
            .text_color(theme::COLOR_TEXT_PRIMARY)
            .text_size(px(theme::FONT_SIZE_MD))
            .child(label)
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme::COLOR_BACKGROUND)
            .text_color(theme::COLOR_TEXT_PRIMARY)
            .track_focus(&self.focus_handle)
            .child(self.render_title_bar(cx))
            .child(self.render_toolbar(cx))
            .child(
                // Main content area: history panel + tree view
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(self.scan_history.clone())
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .overflow_hidden()
                            .child(self.tree_view.clone()),
                    ),
            )
    }
}

impl Focusable for AppView {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}