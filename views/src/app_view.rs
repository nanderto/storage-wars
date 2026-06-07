//! [`AppView`] — root view that orchestrates all sub-components.

use gpui::{
    div, px, App, AppContext, Context, Element, Entity, EventEmitter, IntoElement, ParentElement,
    Render, SharedString, Styled, Window,
};

use crate::drive_selector::{DriveSelector, DriveSelectorEvent};
use crate::scan_history::ScanHistory;
use crate::theme;
use crate::tree_view::TreeView;
use crate::types::DriveInfo;

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

pub enum AppViewEvent {
    ScanRequested(DriveInfo),
}

impl EventEmitter<AppViewEvent> for AppView {}

// ---------------------------------------------------------------------------
// Scan state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanState {
    Idle,
    Scanning,
    Done,
    Error,
}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// Root view: title bar + drive selector + tree view + scan history panel.
pub struct AppView {
    drive_selector: Entity<DriveSelector>,
    tree_view: Entity<TreeView>,
    scan_history: Entity<ScanHistory>,
    scan_state: ScanState,
    selected_drive: Option<DriveInfo>,
}

impl AppView {
    pub fn new() -> Self {
        panic!("AppView must be constructed via AppView::build(cx)")
    }

    pub fn build(cx: &mut Context<Self>) -> Self {
        let drive_selector = cx.new(|cx| DriveSelector::new(cx));
        let tree_view = cx.new(|cx| TreeView::new(cx));
        let scan_history = cx.new(|cx| ScanHistory::new(cx));

        // Subscribe to drive selection events
        cx.subscribe(&drive_selector, |this, _entity, event, cx| {
            if let DriveSelectorEvent::DriveSelected(drive) = event {
                this.selected_drive = Some(drive.clone());
                cx.emit(AppViewEvent::ScanRequested(drive.clone()));
                cx.notify();
            }
        })
        .detach();

        Self {
            drive_selector,
            tree_view,
            scan_history,
            scan_state: ScanState::Idle,
            selected_drive: None,
        }
    }

    // -----------------------------------------------------------------------
    // Title bar
    // -----------------------------------------------------------------------

    fn render_title_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(px(36.0))
            .px(px(theme::SPACING_MD))
            .bg(theme::TITLE_BAR_BG)
            .border_b_1()
            .border_color(theme::BORDER)
            // Allow the title bar to drag the window
            .on_mouse_down(gpui::MouseButton::Left, cx.listener(|_this, _ev, window, _cx| {
                window.start_window_move();
            }))
            .child(
                div()
                    .text_color(theme::TITLE_BAR_TEXT)
                    .text_size(px(theme::FONT_SIZE_MD))
                    .child("Disk Space Analyzer"),
            )
            .child(self.render_window_controls(cx))
    }

    fn render_window_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(theme::SPACING_XS))
            .child(self.window_control_button("✕", cx, |_this, _ev, window, _cx| {
                window.remove_window();
            }))
            .child(self.window_control_button("□", cx, |_this, _ev, window, _cx| {
                window.zoom();
            }))
            .child(self.window_control_button("−", cx, |_this, _ev, window, _cx| {
                window.minimize();
            }))
    }

    fn window_control_button<F>(
        &self,
        label: &'static str,
        cx: &mut Context<Self>,
        handler: F,
    ) -> impl IntoElement
    where
        F: Fn(&mut AppView, &gpui::ClickEvent, &mut Window, &mut Context<AppView>)
            + 'static,
    {
        div()
            .id(SharedString::from(format!("wc-{label}")))
            .w(px(24.0))
            .h(px(24.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(4.0))
            .bg(theme::SURFACE_RAISED)
            .hover(|s| s.bg(theme::BORDER))
            .cursor_pointer()
            .text_color(theme::TEXT_SECONDARY)
            .text_size(px(theme::FONT_SIZE_SM))
            .on_click(cx.listener(handler))
            .child(label)
    }

    // -----------------------------------------------------------------------
    // Toolbar (drive selector + scan button + status)
    // -----------------------------------------------------------------------

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let state_label: SharedString = match self.scan_state {
            ScanState::Idle => "Ready".into(),
            ScanState::Scanning => "Scanning…".into(),
            ScanState::Done => "Scan complete".into(),
            ScanState::Error => "Scan failed".into(),
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(theme::SPACING_SM))
            .px(px(theme::SPACING_MD))
            .py(px(theme::SPACING_SM))
            .bg(theme::SURFACE)
            .border_b_1()
            .border_color(theme::BORDER)
            .child(
                div()
                    .flex_1()
                    .child(self.drive_selector.clone()),
            )
            .child(
                // Scan button
                div()
                    .id("btn-scan")
                    .px(px(theme::SPACING_LG))
                    .py(px(theme::SPACING_XS))
                    .bg(theme::ACCENT)
                    .hover(|s| s.bg(theme::ACCENT_HOVER))
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .text_color(theme::TEXT_PRIMARY)
                    .text_size(px(theme::FONT_SIZE_MD))
                    .on_click(cx.listener(|this, _ev, _window, cx| {
                        if let Some(drive) = this.selected_drive.clone() {
                            this.scan_state = ScanState::Scanning;
                            cx.emit(AppViewEvent::ScanRequested(drive));
                            cx.notify();
                        }
                    }))
                    .child("Scan"),
            )
            .child(
                div()
                    .text_color(theme::TEXT_MUTED)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(state_label),
            )
    }
}

impl Default for AppView {
    fn default() -> Self {
        panic!("AppView must be constructed via AppView::build(cx)")
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title_bar = self.render_title_bar(cx);
        let toolbar = self.render_toolbar(cx);

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme::BACKGROUND)
            .child(title_bar)
            .child(toolbar)
            .child(
                // Main content area
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .overflow_hidden()
                    .child(self.scan_history.clone())
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .child(self.tree_view.clone()),
                    ),
            )
    }
}