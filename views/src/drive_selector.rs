//! [`DriveSelector`] — a focusable drive-selection widget.

use gpui::{
    div, px, AnyElement, App, AppContext, Context, Element, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled,
    Window,
};

use crate::theme;
use crate::types::DriveInfo;

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

pub enum DriveSelectorEvent {
    /// Emitted when the user selects a drive.
    DriveSelected(DriveInfo),
}

impl EventEmitter<DriveSelectorEvent> for DriveSelector {}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// A focusable drop-down–style widget that lists available drives.
pub struct DriveSelector {
    drives: Vec<DriveInfo>,
    selected_index: Option<usize>,
    is_open: bool,
    focus_handle: FocusHandle,
}

impl DriveSelector {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            drives: Vec::new(),
            selected_index: None,
            is_open: false,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Replace the drive list and reset selection.
    pub fn set_drives(&mut self, drives: Vec<DriveInfo>) {
        self.drives = drives;
        self.selected_index = None;
        self.is_open = false;
    }

    /// Returns the currently selected drive, if any.
    pub fn selected_drive(&self) -> Option<&DriveInfo> {
        self.selected_index.and_then(|i| self.drives.get(i))
    }

    fn toggle_open(&mut self) {
        self.is_open = !self.is_open;
    }

    fn select_index(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.drives.len() {
            self.selected_index = Some(index);
            self.is_open = false;
            let drive = self.drives[index].clone();
            cx.emit(DriveSelectorEvent::DriveSelected(drive));
        }
    }

    fn render_trigger(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            .px(px(theme::SPACING_SM))
            .py(px(theme::SPACING_XS))
            .bg(theme::SURFACE)
            .border_1()
            .border_color(if is_open { theme::ACCENT } else { theme::BORDER })
            .rounded(px(4.0))
            .cursor_pointer()
            .on_click(cx.listener(|this, _ev, _window, cx| {
                this.toggle_open();
                cx.notify();
            }))
            .child(
                div()
                    .text_color(theme::TEXT_PRIMARY)
                    .text_size(px(theme::FONT_SIZE_MD))
                    .child(label),
            )
            .child(
                div()
                    .text_color(theme::TEXT_SECONDARY)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(if is_open { "▲" } else { "▼" }),
            )
    }

    fn render_dropdown(&self, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        if !self.is_open {
            return None;
        }

        let items: Vec<AnyElement> = self
            .drives
            .iter()
            .enumerate()
            .map(|(i, drive)| {
                let label: SharedString = drive.display_label().into();
                let is_selected = self.selected_index == Some(i);

                div()
                    .id(("drive-option", i))
                    .px(px(theme::SPACING_SM))
                    .py(px(theme::SPACING_XS))
                    .bg(if is_selected {
                        theme::HISTORY_ITEM_SELECTED
                    } else {
                        theme::SURFACE
                    })
                    .hover(|s| s.bg(theme::HISTORY_ITEM_HOVER))
                    .cursor_pointer()
                    .text_color(theme::TEXT_PRIMARY)
                    .text_size(px(theme::FONT_SIZE_MD))
                    .on_click(cx.listener(move |this, _ev, _window, cx| {
                        this.select_index(i, cx);
                        cx.notify();
                    }))
                    .child(label)
                    .into_any_element()
            })
            .collect();

        Some(
            div()
                .absolute()
                .top(px(32.0))
                .left(px(0.0))
                .right(px(0.0))
                .z_index(100)
                .bg(theme::SURFACE)
                .border_1()
                .border_color(theme::BORDER)
                .rounded(px(4.0))
                .shadow_lg()
                .overflow_y_scroll()
                .max_h(px(240.0))
                .children(items),
        )
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let trigger = self.render_trigger(cx);
        let dropdown = self.render_dropdown(cx);

        let mut container = div()
            .relative()
            .w_full()
            .track_focus(&self.focus_handle)
            .child(trigger);

        if let Some(dd) = dropdown {
            container = container.child(dd);
        }

        container
    }
}