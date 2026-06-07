use gpui::{
    actions, div, px, AppContext, Div, EventEmitter, FocusHandle, FocusableView, IntoElement,
    KeyBinding, ParentElement, Render, SharedString, Styled, View, ViewContext, VisualContext,
    WindowContext,
};

use crate::types::{DriveInfo, format_bytes};
use crate::ui_helpers::{colors, font_size, spacing};

/// Events emitted by the DriveSelector
#[derive(Debug, Clone)]
pub enum DriveSelectorEvent {
    DriveSelected(String),
}

/// A focusable drive selection widget
pub struct DriveSelector {
    drives: Vec<DriveInfo>,
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

    pub fn set_drives(&mut self, drives: Vec<DriveInfo>, cx: &mut ViewContext<Self>) {
        self.drives = drives;
        if self.selected_index.map_or(false, |i| i >= self.drives.len()) {
            self.selected_index = None;
        }
        cx.notify();
    }

    pub fn selected_drive(&self) -> Option<&DriveInfo> {
        self.selected_index.and_then(|i| self.drives.get(i))
    }

    pub fn select_drive_by_id(&mut self, id: &str, cx: &mut ViewContext<Self>) {
        if let Some(idx) = self.drives.iter().position(|d| d.id == id) {
            self.selected_index = Some(idx);
            cx.notify();
        }
    }

    fn toggle_open(&mut self, cx: &mut ViewContext<Self>) {
        self.is_open = !self.is_open;
        cx.notify();
    }

    fn select_index(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        if index < self.drives.len() {
            self.selected_index = Some(index);
            self.is_open = false;
            let drive_id = self.drives[index].id.clone();
            cx.emit(DriveSelectorEvent::DriveSelected(drive_id));
            cx.notify();
        }
    }

    fn render_selected_label(&self) -> SharedString {
        match self.selected_index.and_then(|i| self.drives.get(i)) {
            Some(drive) => drive.display_label().into(),
            None => "Select a drive…".into(),
        }
    }

    fn render_dropdown_item(&self, index: usize, drive: &DriveInfo, cx: &mut ViewContext<Self>) -> Div {
        let is_selected = self.selected_index == Some(index);
        let label = drive.display_label();
        let usage = format!("{:.0}% used", drive.usage_percent());

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .px(spacing::md())
            .py(spacing::xs())
            .cursor_pointer()
            .bg(if is_selected { colors::overlay() } else { colors::surface() })
            .hover(|s| s.bg(colors::overlay()))
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(move |this, _, cx| {
                    this.select_index(index, cx);
                }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_color(colors::text())
                            .text_size(font_size::sm())
                            .child(SharedString::from(label)),
                    )
                    .child(
                        div()
                            .text_color(colors::muted())
                            .text_size(font_size::xs())
                            .child(SharedString::from(usage)),
                    ),
            )
            .child(if is_selected {
                div()
                    .text_color(colors::accent())
                    .text_size(font_size::sm())
                    .child("✓")
            } else {
                div()
            })
    }
}

impl EventEmitter<DriveSelectorEvent> for DriveSelector {}

impl FocusableView for DriveSelector {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DriveSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(cx);
        let is_open = self.is_open;
        let selected_label = self.render_selected_label();

        let mut container = div()
            .relative()
            .w_full()
            .track_focus(&self.focus_handle);

        // Main selector button
        let selector_button = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .px(spacing::md())
            .py(spacing::sm())
            .rounded(px(6.0))
            .bg(colors::surface())
            .border_1()
            .border_color(if is_focused || is_open {
                colors::accent()
            } else {
                colors::border()
            })
            .cursor_pointer()
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _, cx| {
                    this.toggle_open(cx);
                }),
            )
            .child(
                div()
                    .text_color(colors::text())
                    .text_size(font_size::sm())
                    .child(selected_label),
            )
            .child(
                div()
                    .text_color(colors::muted())
                    .text_size(font_size::xs())
                    .child(if is_open { "▲" } else { "▼" }),
            );

        container = container.child(selector_button);

        // Dropdown list
        if is_open && !self.drives.is_empty() {
            let mut dropdown = div()
                .absolute()
                .top(px(40.0))
                .left(px(0.0))
                .right(px(0.0))
                .z_index(100)
                .bg(colors::surface())
                .border_1()
                .border_color(colors::border())
                .rounded(px(6.0))
                .shadow_lg()
                .overflow_hidden()
                .flex()
                .flex_col();

            for (i, drive) in self.drives.clone().iter().enumerate() {
                dropdown = dropdown.child(self.render_dropdown_item(i, drive, cx));
            }

            container = container.child(dropdown);
        }

        container
    }
}