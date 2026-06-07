//! Drive selector widget — a focusable dropdown for choosing a drive/volume.

use gpui::{
    div, px, AppContext, Div, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window,
};

use crate::{
    theme,
    types::{DriveInfo},
};

/// Focusable drive-selection widget.
pub struct DriveSelector {
    focus_handle: FocusHandle,
    drives: Vec<DriveInfo>,
    selected_index: Option<usize>,
    is_open: bool,
}

impl DriveSelector {
    pub fn new(drives: Vec<DriveInfo>, cx: &mut gpui::Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            drives,
            selected_index: None,
            is_open: false,
        }
    }

    pub fn selected_drive(&self) -> Option<&DriveInfo> {
        self.selected_index.and_then(|i| self.drives.get(i))
    }

    fn render_selected_label(&self) -> String {
        match self.selected_drive() {
            Some(d) => d.display_label(),
            None => "Select a drive…".into(),
        }
    }

    fn render_dropdown_item(&self, idx: usize, drive: &DriveInfo) -> Div {
        let is_selected = self.selected_index == Some(idx);
        div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(theme::DRIVE_SELECTOR_HEIGHT))
            .px(px(theme::SPACING_MD))
            .bg(if is_selected {
                theme::COLOR_SELECTED
            } else {
                theme::COLOR_SURFACE_RAISED
            })
            .text_color(if is_selected {
                theme::COLOR_TEXT_PRIMARY
            } else {
                theme::COLOR_TEXT_SECONDARY
            })
            .text_size(px(theme::FONT_SIZE_SM))
            .child(drive.display_label())
    }
}

impl Render for DriveSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let label = self.render_selected_label();
        let is_open = self.is_open;

        let mut container = div()
            .flex()
            .flex_col()
            .relative()
            .h(px(theme::DRIVE_SELECTOR_HEIGHT))
            .min_w(px(240.0))
            .track_focus(&self.focus_handle);

        // Trigger button
        let trigger = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(theme::DRIVE_SELECTOR_HEIGHT))
            .px(px(theme::SPACING_MD))
            .rounded(px(4.0))
            .bg(theme::COLOR_SURFACE_RAISED)
            .border_1()
            .border_color(if is_open {
                theme::COLOR_SELECTED_BORDER
            } else {
                theme::COLOR_BORDER
            })
            .text_color(theme::COLOR_TEXT_PRIMARY)
            .text_size(px(theme::FONT_SIZE_MD))
            .child(label)
            .child(
                div()
                    .text_color(theme::COLOR_TEXT_MUTED)
                    .text_size(px(theme::FONT_SIZE_SM))
                    .child(if is_open { "▲" } else { "▼" }),
            );

        container = container.child(trigger);

        // Dropdown list (shown when open)
        if is_open {
            let items: Vec<Div> = self
                .drives
                .iter()
                .enumerate()
                .map(|(i, d)| self.render_dropdown_item(i, d))
                .collect();

            let dropdown = div()
                .absolute()
                .top(px(theme::DRIVE_SELECTOR_HEIGHT))
                .left(px(0.0))
                .w_full()
                .bg(theme::COLOR_SURFACE_RAISED)
                .border_1()
                .border_color(theme::COLOR_BORDER)
                .rounded(px(4.0))
                .shadow_lg()
                .z_index(100)
                .children(items);

            container = container.child(dropdown);
        }

        container
    }
}

impl Focusable for DriveSelector {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}