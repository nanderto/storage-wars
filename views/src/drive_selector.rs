use gpui::{
    div, px, App, Context, Element, EventEmitter, Focusable, FocusHandle, InteractiveElement,
    IntoElement, ParentElement, Render, SharedString, Styled, Window,
};
use uuid::Uuid;

use crate::theme::current_theme;
use crate::types::DriveInfo;

/// Events emitted when the user selects a drive.
pub enum DriveSelectorEvent {
    DriveSelected(Uuid),
}

impl EventEmitter<DriveSelectorEvent> for DriveSelector {}

/// A focusable drive-selection widget.
pub struct DriveSelector {
    focus_handle: FocusHandle,
    drives: Vec<DriveInfo>,
    selected_id: Option<Uuid>,
    is_open: bool,
}

impl DriveSelector {
    pub fn new(
        drives: Vec<DriveInfo>,
        selected_id: Option<Uuid>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            drives,
            selected_id,
            is_open: false,
        }
    }

    pub fn set_drives(&mut self, drives: Vec<DriveInfo>, cx: &mut Context<Self>) {
        self.drives = drives;
        cx.notify();
    }

    pub fn set_selected(&mut self, id: Option<Uuid>, cx: &mut Context<Self>) {
        self.selected_id = id;
        cx.notify();
    }

    fn selected_drive(&self) -> Option<&DriveInfo> {
        self.selected_id
            .and_then(|id| self.drives.iter().find(|d| d.id == id))
    }

    fn format_drive_label(drive: &DriveInfo) -> String {
        let used_gb = drive.used_bytes as f64 / 1_073_741_824.0;
        let total_gb = drive.total_bytes as f64 / 1_073_741_824.0;
        let label = drive.display_label();
        format!("{label}  —  {used_gb:.1} GB / {total_gb:.1} GB used")
    }
}

impl Focusable for DriveSelector {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DriveSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = current_theme();
        let is_focused = self.focus_handle.is_focused(cx);

        let selected_label = self
            .selected_drive()
            .map(Self::format_drive_label)
            .unwrap_or_else(|| "Select a drive…".to_string());

        let border_color = if is_focused {
            theme.border_focused
        } else {
            theme.border
        };

        div()
            .id("drive-selector")
            .flex()
            .flex_col()
            .w_full()
            .child(
                // Trigger button
                div()
                    .id("drive-selector-trigger")
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .h(px(36.0))
                    .px(px(12.0))
                    .bg(theme.surface_elevated)
                    .border_1()
                    .border_color(border_color)
                    .rounded(px(6.0))
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.is_open = !this.is_open;
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .overflow_hidden()
                            .child(SharedString::from(selected_label)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(if self.is_open { "▲" } else { "▼" }),
                    ),
            )
            .when(self.is_open, |el| {
                let drives: Vec<DriveInfo> = self.drives.clone();
                let selected_id = self.selected_id;

                el.child(
                    div()
                        .id("drive-selector-dropdown")
                        .absolute()
                        .mt(px(40.0))
                        .w_full()
                        .bg(theme.surface_elevated)
                        .border_1()
                        .border_color(theme.border)
                        .rounded(px(6.0))
                        .shadow_lg()
                        .z_index(100)
                        .overflow_hidden()
                        .children(drives.into_iter().enumerate().map(|(i, drive)| {
                            let is_selected = selected_id == Some(drive.id);
                            let drive_id = drive.id;
                            let label = Self::format_drive_label(&drive);

                            div()
                                .id(("drive-option", i))
                                .flex()
                                .flex_row()
                                .items_center()
                                .h(px(36.0))
                                .px(px(12.0))
                                .cursor_pointer()
                                .bg(if is_selected {
                                    theme.selection_bg
                                } else {
                                    theme.surface_elevated
                                })
                                .hover(|s| s.bg(theme.scan_history_item_hover))
                                .on_click(cx.listener(move |this, _, _window, cx| {
                                    this.selected_id = Some(drive_id);
                                    this.is_open = false;
                                    cx.emit(DriveSelectorEvent::DriveSelected(drive_id));
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(if is_selected {
                                            theme.text_primary
                                        } else {
                                            theme.text_secondary
                                        })
                                        .child(SharedString::from(label)),
                                )
                        })),
                )
            })
    }
}