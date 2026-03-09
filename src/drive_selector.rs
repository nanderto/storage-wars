use gpui::prelude::*;
use gpui::{
    div, px, rgb, App, ClickEvent, Context, EventEmitter, Focusable, FocusHandle,
    IntoElement, Render, SharedString, Window,
};

use crate::models::{format_size, DriveInfo};

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

pub enum DriveSelectorEvent {
    DriveSelected(String),
}

impl EventEmitter<DriveSelectorEvent> for DriveSelector {}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

pub struct DriveSelector {
    pub drives: Vec<DriveInfo>,
    pub selected_drive: Option<String>,
    focus_handle: FocusHandle,
}

impl DriveSelector {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            drives: Vec::new(),
            selected_drive: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_drives(&mut self, drives: Vec<DriveInfo>, cx: &mut Context<Self>) {
        self.drives = drives;
        cx.notify();
    }
}

impl Focusable for DriveSelector {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DriveSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let drives = self.drives.clone();
        let selected = self.selected_drive.clone();

        div()
            .flex()
            .flex_col()
            .w(px(200.))
            .h_full()
            .bg(rgb(0x1e1e2e))
            .border_r_1()
            .border_color(rgb(0x313244))
            .child(
                div()
                    .px_3()
                    .py_2()
                    .text_color(rgb(0xa6adc8))
                    .text_sm()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("DRIVES"),
            )
            .children(drives.into_iter().enumerate().map(|(ix, drive)| {
                let is_selected = selected.as_deref() == Some(drive.name.as_str());
                let name = drive.name.clone();
                let label: SharedString = format!(
                    "{} — {} free",
                    drive.name,
                    format_size(drive.available_space)
                )
                .into();

                div()
                    .id(ix)
                    .px_3()
                    .py_2()
                    .cursor_pointer()
                    .text_color(if is_selected {
                        rgb(0xcdd6f4)
                    } else {
                        rgb(0xa6adc8)
                    })
                    .when(is_selected, |el| el.bg(rgb(0x313244)))
                    .hover(|s| s.bg(rgb(0x45475a)))
                    .child(label)
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        this.selected_drive = Some(name.clone());
                        cx.emit(DriveSelectorEvent::DriveSelected(name.clone()));
                        cx.notify();
                    }))
            }))
    }
}
