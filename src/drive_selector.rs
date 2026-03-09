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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DriveInfo;
    use std::{cell::RefCell, rc::Rc};

    fn drives() -> Vec<DriveInfo> {
        vec![
            DriveInfo { name: "C:".into(), total_space: 500_000, available_space: 100_000 },
            DriveInfo { name: "D:".into(), total_space: 1_000_000, available_space: 400_000 },
        ]
    }

    #[gpui::test]
    fn initial_state_is_empty(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| DriveSelector::new(cx));
        view.read_with(cx, |v, _| {
            assert!(v.drives.is_empty());
            assert!(v.selected_drive.is_none());
        });
    }

    #[gpui::test]
    fn set_drives_updates_list(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| DriveSelector::new(cx));
        view.update(cx, |v, cx| v.set_drives(drives(), cx));
        cx.run_until_parked();
        view.read_with(cx, |v, _| {
            assert_eq!(v.drives.len(), 2);
            assert_eq!(v.drives[0].name, "C:");
            assert_eq!(v.drives[1].name, "D:");
        });
    }

    #[gpui::test]
    fn selected_drive_can_be_set(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| DriveSelector::new(cx));
        view.update(cx, |v, cx| {
            v.set_drives(drives(), cx);
            v.selected_drive = Some("C:".into());
            cx.notify();
        });
        cx.run_until_parked();
        view.read_with(cx, |v, _| {
            assert_eq!(v.selected_drive, Some("C:".to_string()));
        });
    }

    #[gpui::test]
    fn drive_selected_emits_event(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| DriveSelector::new(cx));
        view.update(cx, |v, cx| v.set_drives(drives(), cx));

        let selected: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
        let captured = selected.clone();
        let _sub = cx.update(|_window, app| {
            app.subscribe(&view, move |_, event: &DriveSelectorEvent, _| {
                if let DriveSelectorEvent::DriveSelected(name) = event {
                    captured.borrow_mut().push(name.clone());
                }
            })
        });

        view.update(cx, |v, cx| {
            let name = "D:".to_string();
            v.selected_drive = Some(name.clone());
            cx.emit(DriveSelectorEvent::DriveSelected(name));
            cx.notify();
        });
        cx.run_until_parked();

        assert_eq!(*selected.borrow(), vec!["D:".to_string()]);
    }

    #[gpui::test]
    fn render_does_not_panic(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_, cx| DriveSelector::new(cx));
        view.update(cx, |v, cx| v.set_drives(drives(), cx));
        cx.run_until_parked();
        // If we reach here without panicking the render is sound.
    }
}
