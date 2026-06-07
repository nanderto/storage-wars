//! [`DriveSelector`] — a focusable drop-down widget for choosing a drive.

use gpui::*;

use crate::theme;
use crate::types::Drive;

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// Actions emitted by [`DriveSelector`].
#[derive(Debug, Clone)]
pub struct DriveSelected(pub Drive);

impl EventEmitter<DriveSelected> for DriveSelector {}

/// A focusable drive-selection widget.
pub struct DriveSelector {
    drives: Vec<Drive>,
    selected_index: Option<usize>,
    is_open: bool,
    focus_handle: FocusHandle,
}

impl DriveSelector {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            drives: Vec::new(),
            selected_index: None,
            is_open: false,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Replace the drive list (e.g. after a refresh).
    pub fn set_drives(&mut self, drives: Vec<Drive>, cx: &mut ViewContext<Self>) {
        self.drives = drives;
        self.selected_index = if self.drives.is_empty() { None } else { Some(0) };
        cx.notify();
    }

    /// Currently selected drive, if any.
    pub fn selected_drive(&self) -> Option<&Drive> {
        self.selected_index.and_then(|i| self.drives.get(i))
    }

    fn toggle_open(&mut self, cx: &mut ViewContext<Self>) {
        self.is_open = !self.is_open;
        cx.notify();
    }

    fn select_index(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        self.selected_index = Some(index);
        self.is_open = false;
        if let Some(drive) = self.drives.get(index).cloned() {
            cx.emit(DriveSelected(drive));
        }
        cx.notify();
    }

    fn render_trigger(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let label: SharedString = self
            .selected_drive()
            .map(|d| d.display_label())
            .unwrap_or_else(|| "Select a drive…".to_string())
            .into();

        let is_open = self.is_open;

        div()
            .id("drive-selector-trigger")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(32.0))
            .px(px(10.0))
            .rounded(px(4.0))
            .bg(theme::bg_secondary())
            .border_1()
            .border_color(theme::border())
            .cursor_pointer()
            .on_click(cx.listener(|this, _ev, cx| this.toggle_open(cx)))
            .child(
                div()
                    .text_color(theme::text_primary())
                    .text_sm()
                    .overflow_hidden()
                    .child(label),
            )
            .child(
                div()
                    .text_color(theme::text_secondary())
                    .text_sm()
                    .child(if is_open { "▲" } else { "▼" }),
            )
    }

    fn render_dropdown(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let items: Vec<_> = self
            .drives
            .iter()
            .enumerate()
            .map(|(i, drive)| {
                let label: SharedString = drive.display_label().into();
                let is_selected = self.selected_index == Some(i);

                div()
                    .id(ElementId::Name(format!("drive-option-{i}").into()))
                    .flex()
                    .items_center()
                    .w_full()
                    .h(px(30.0))
                    .px(px(10.0))
                    .cursor_pointer()
                    .bg(if is_selected {
                        theme::bg_selected()
                    } else {
                        theme::bg_secondary()
                    })
                    .hover(|s| s.bg(theme::bg_hover()))
                    .on_click(cx.listener(move |this, _ev, cx| this.select_index(i, cx)))
                    .child(
                        div()
                            .text_color(theme::text_primary())
                            .text_sm()
                            .child(label),
                    )
            })
            .collect();

        div()
            .absolute()
            .top(px(34.0))
            .left(px(0.0))
            .w_full()
            .z_index(50)
            .rounded(px(4.0))
            .border_1()
            .border_color(theme::border())
            .bg(theme::bg_secondary())
            .shadow_lg()
            .overflow_hidden()
            .children(items)
    }
}

// ---------------------------------------------------------------------------
// Focusable
// ---------------------------------------------------------------------------

impl Focusable for DriveSelector {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for DriveSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_open = self.is_open;

        div()
            .id("drive-selector")
            .relative()
            .w_full()
            .track_focus(&self.focus_handle)
            .child(self.render_trigger(cx))
            .when(is_open, |el| el.child(self.render_dropdown(cx)))
    }
}