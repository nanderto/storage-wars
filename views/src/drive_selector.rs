//! Drive selector widget – a focusable `Select`-style component.

use gpui::{
    div, px, Context, FocusHandle, Focusable, IntoElement, ParentElement, Render, Styled,
    ViewContext,
};

use crate::theme::Palette;

/// Represents a single drive entry shown in the selector.
#[derive(Debug, Clone)]
pub struct DriveEntry {
    /// Drive letter or mount point (e.g. `"C:\\"` or `"/dev/sda1"`).
    pub path: String,
    /// Optional volume label (e.g. `"System"`).
    pub volume_label: Option<String>,
    /// Total capacity in bytes.
    pub total_bytes: u64,
    /// Available free space in bytes.
    pub free_bytes: u64,
}

impl DriveEntry {
    /// Formats the drive label for display.
    ///
    /// Format: `"<path> (<volume>) – <used>/<total>"`  
    /// When no volume label is present: `"<path> – <used>/<total>"`
    pub fn display_label(&self) -> String {
        let used = self.total_bytes.saturating_sub(self.free_bytes);
        let label_part = match &self.volume_label {
            Some(v) if !v.is_empty() => format!("{} ({})", self.path, v),
            _ => self.path.clone(),
        };
        format!(
            "{} – {} / {}",
            label_part,
            format_bytes(used),
            format_bytes(self.total_bytes)
        )
    }
}

/// Formats a byte count into a human-readable string (GiB / MiB / KiB / B).
pub fn format_bytes(bytes: u64) -> String {
    const GIB: u64 = 1 << 30;
    const MIB: u64 = 1 << 20;
    const KIB: u64 = 1 << 10;

    if bytes >= GIB {
        format!("{:.1} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// A focusable drive selector widget.
pub struct DriveSelector {
    focus_handle: FocusHandle,
    drives: Vec<DriveEntry>,
    selected_index: usize,
    is_open: bool,
}

impl DriveSelector {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            drives: Vec::new(),
            selected_index: 0,
            is_open: false,
        }
    }

    /// Replaces the drive list and resets selection.
    pub fn set_drives(&mut self, drives: Vec<DriveEntry>, cx: &mut ViewContext<Self>) {
        self.drives = drives;
        self.selected_index = 0;
        self.is_open = false;
        cx.notify();
    }

    /// Returns the currently selected drive, if any.
    pub fn selected_drive(&self) -> Option<&DriveEntry> {
        self.drives.get(self.selected_index)
    }

    fn toggle_open(&mut self, cx: &mut ViewContext<Self>) {
        self.is_open = !self.is_open;
        cx.notify();
    }

    fn select_index(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        if index < self.drives.len() {
            self.selected_index = index;
            self.is_open = false;
            cx.notify();
        }
    }
}

impl Focusable for DriveSelector {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DriveSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let selected_label = self
            .drives
            .get(self.selected_index)
            .map(|d| d.display_label())
            .unwrap_or_else(|| "No drives found".to_string());

        let is_focused = self.focus_handle.is_focused(cx);

        div()
            .flex()
            .flex_col()
            .relative()
            // ── Trigger button ──────────────────────────────────────────
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(px(32.0))
                    .min_w(px(280.0))
                    .px(px(10.0))
                    .bg(Palette::surface_elevated())
                    .border_1()
                    .border_color(if is_focused {
                        Palette::accent()
                    } else {
                        Palette::border()
                    })
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .on_mouse_down(gpui::MouseButton::Left, cx.listener(|this, _, cx| {
                        this.toggle_open(cx);
                    }))
                    .child(
                        div()
                            .text_color(Palette::text_primary())
                            .text_sm()
                            .overflow_hidden()
                            .child(selected_label),
                    )
                    .child(
                        div()
                            .text_color(Palette::text_secondary())
                            .text_xs()
                            .child(if self.is_open { "▲" } else { "▼" }),
                    ),
            )
            // ── Dropdown list ───────────────────────────────────────────
            .when(self.is_open && !self.drives.is_empty(), |el| {
                let items: Vec<_> = self
                    .drives
                    .iter()
                    .enumerate()
                    .map(|(i, drive)| {
                        let label = drive.display_label();
                        let is_selected = i == self.selected_index;
                        div()
                            .flex()
                            .items_center()
                            .h(px(32.0))
                            .px(px(10.0))
                            .bg(if is_selected {
                                Palette::selection()
                            } else {
                                Palette::surface_elevated()
                            })
                            .hover(|s| s.bg(Palette::surface()))
                            .cursor_pointer()
                            .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _, cx| {
                                this.select_index(i, cx);
                            }))
                            .child(
                                div()
                                    .text_color(Palette::text_primary())
                                    .text_sm()
                                    .child(label),
                            )
                    })
                    .collect();

                el.child(
                    div()
                        .absolute()
                        .top(px(34.0))
                        .left(px(0.0))
                        .min_w(px(280.0))
                        .bg(Palette::surface_elevated())
                        .border_1()
                        .border_color(Palette::border())
                        .rounded(px(4.0))
                        .shadow_lg()
                        .z_index(100)
                        .flex()
                        .flex_col()
                        .children(items),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_label_with_volume() {
        let entry = DriveEntry {
            path: "C:\\".to_string(),
            volume_label: Some("System".to_string()),
            total_bytes: 500 * (1 << 30),
            free_bytes: 100 * (1 << 30),
        };
        let label = entry.display_label();
        assert!(label.contains("C:\\"));
        assert!(label.contains("System"));
        assert!(label.contains("GiB"));
    }

    #[test]
    fn display_label_without_volume() {
        let entry = DriveEntry {
            path: "/dev/sda1".to_string(),
            volume_label: None,
            total_bytes: 1 << 30,
            free_bytes: 512 * (1 << 20),
        };
        let label = entry.display_label();
        assert!(label.contains("/dev/sda1"));
        assert!(!label.contains("()"));
    }

    #[test]
    fn format_bytes_gib() {
        assert_eq!(format_bytes(1 << 30), "1.0 GiB");
    }

    #[test]
    fn format_bytes_mib() {
        assert_eq!(format_bytes(1 << 20), "1.0 MiB");
    }

    #[test]
    fn format_bytes_kib() {
        assert_eq!(format_bytes(1 << 10), "1.0 KiB");
    }

    #[test]
    fn format_bytes_b() {
        assert_eq!(format_bytes(512), "512 B");
    }
}