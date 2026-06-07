//! Drive selector widget — a focusable `Select`-style component.

use gpui::{
    div, prelude::*, px, Context, Element, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};

use crate::theme::{
    COLOR_ACCENT, COLOR_BORDER, COLOR_SURFACE, COLOR_TEXT_PRIMARY, COLOR_TEXT_SECONDARY,
};

/// A single drive entry shown in the selector.
#[derive(Debug, Clone)]
pub struct DriveEntry {
    /// Drive path, e.g. `C:\` or `/dev/sda1`.
    pub path: String,
    /// Optional volume label, e.g. `"System"`.
    pub volume_label: Option<String>,
    /// Free space in bytes.
    pub free_bytes: u64,
    /// Total capacity in bytes.
    pub total_bytes: u64,
}

impl DriveEntry {
    /// Formats the drive label as `"<label> (<path>) — X.X / Y.Y GB"`.
    ///
    /// When no volume label is present the path is used as the primary name.
    pub fn display_label(&self) -> String {
        let name = match &self.volume_label {
            Some(label) if !label.is_empty() => format!("{} ({})", label, self.path),
            _ => self.path.clone(),
        };
        let free_gb = self.free_bytes as f64 / 1_073_741_824.0;
        let total_gb = self.total_bytes as f64 / 1_073_741_824.0;
        format!("{} — {:.1} / {:.1} GB", name, free_gb, total_gb)
    }
}

/// Focusable drive-selection widget.
pub struct DriveSelector {
    focus_handle: FocusHandle,
    drives: Vec<DriveEntry>,
    selected_index: usize,
    is_open: bool,
}

impl DriveSelector {
    /// Constructs the widget within a GPUI context.
    pub fn build(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            drives: Vec::new(),
            selected_index: 0,
            is_open: false,
        }
    }

    /// Replaces the current drive list.
    pub fn set_drives(&mut self, drives: Vec<DriveEntry>) {
        self.drives = drives;
        self.selected_index = 0;
    }

    /// Returns the currently selected drive, if any.
    pub fn selected_drive(&self) -> Option<&DriveEntry> {
        self.drives.get(self.selected_index)
    }
}

impl Focusable for DriveSelector {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DriveSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let label = self
            .selected_drive()
            .map(|d| d.display_label())
            .unwrap_or_else(|| "No drives found".to_string());

        div()
            .flex()
            .flex_row()
            .items_center()
            .min_w(px(240.0))
            .h(px(28.0))
            .px(px(8.0))
            .bg(COLOR_SURFACE)
            .border_1()
            .border_color(if self.is_open { COLOR_ACCENT } else { COLOR_BORDER })
            .rounded(px(4.0))
            .text_color(COLOR_TEXT_PRIMARY)
            .child(div().flex_1().overflow_hidden().child(label))
            .child(
                div()
                    .pl(px(8.0))
                    .text_color(COLOR_TEXT_SECONDARY)
                    .child("▾"),
            )
    }
}