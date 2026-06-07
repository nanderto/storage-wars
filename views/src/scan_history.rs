//! [`ScanHistory`] — 280 px-wide focusable panel showing scan snapshots.

use gpui::{
    div, px, AnyElement, AppContext, Context, Element, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement, IntoElement, ParentElement, Render, SharedString, Styled,
    Window,
};
use uuid::Uuid;

use crate::theme;
use crate::types::ScanSnapshot;

// ---------------------------------------------------------------------------
// Selection state
// ---------------------------------------------------------------------------

/// Which role a snapshot is assigned in a comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotRole {
    Base,
    New,
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

pub enum ScanHistoryEvent {
    Compare {
        base_id: Uuid,
        new_id: Uuid,
    },
    Delete(Uuid),
    SelectionChanged {
        id: Uuid,
        role: SnapshotRole,
    },
}

impl EventEmitter<ScanHistoryEvent> for ScanHistory {}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// A 280 px-wide focusable panel listing scan snapshots.
pub struct ScanHistory {
    snapshots: Vec<ScanSnapshot>,
    base_id: Option<Uuid>,
    new_id: Option<Uuid>,
    focus_handle: FocusHandle,
}

impl ScanHistory {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            snapshots: Vec::new(),
            base_id: None,
            new_id: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_snapshots(&mut self, snapshots: Vec<ScanSnapshot>) {
        self.snapshots = snapshots;
        self.base_id = None;
        self.new_id = None;
    }

    fn can_compare(&self) -> bool {
        self.base_id.is_some() && self.new_id.is_some() && self.base_id != self.new_id
    }

    fn handle_compare(&mut self, cx: &mut Context<Self>) {
        if let (Some(base_id), Some(new_id)) = (self.base_id, self.new_id) {
            cx.emit(ScanHistoryEvent::Compare { base_id, new_id });
        }
    }

    fn handle_delete(&mut self, id: Uuid, cx: &mut Context<Self>) {
        self.snapshots.retain(|s| s.id != id);
        if self.base_id == Some(id) {
            self.base_id = None;
        }
        if self.new_id == Some(id) {
            self.new_id = None;
        }
        cx.emit(ScanHistoryEvent::Delete(id));
        cx.notify();
    }

    fn select_as_base(&mut self, id: Uuid, cx: &mut Context<Self>) {
        self.base_id = Some(id);
        cx.emit(ScanHistoryEvent::SelectionChanged {
            id,
            role: SnapshotRole::Base,
        });
        cx.notify();
    }

    fn select_as_new(&mut self, id: Uuid, cx: &mut Context<Self>) {
        self.new_id = Some(id);
        cx.emit(ScanHistoryEvent::SelectionChanged {
            id,
            role: SnapshotRole::New,
        });
        cx.notify();
    }

    fn render_snapshot_row(
        &self,
        snapshot: &ScanSnapshot,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let id = snapshot.id;
        let label: SharedString = snapshot
            .scanned_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .into();

        let is_base = self.base_id == Some(id);
        let is_new = self.new_id == Some(id);

        let bg = if is_base || is_new {
            theme::HISTORY_ITEM_SELECTED
        } else {
            theme::HISTORY_BG
        };

        div()
            .id(("scan-row", id.as_u128() as usize))
            .flex()
            .flex_col()
            .px(px(theme::SPACING_SM))
            .py(px(theme::SPACING_XS))
            .bg(bg)
            .hover(|s| s.bg(theme::HISTORY_ITEM_HOVER))
            .border_b_1()
            .border_color(theme::BORDER)
            .child(
                div()
                    .text_color(theme::TEXT_PRIMARY)
                    .text_size(px(theme::FONT_SIZE_MD))
                    .child(label),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(theme::SPACING_XS))
                    .mt(px(theme::SPACING_XS))
                    .child(
                        // Base button
                        div()
                            .id(("btn-base", id.as_u128() as usize))
                            .px(px(theme::SPACING_XS))
                            .py(px(2.0))
                            .bg(if is_base { theme::ACCENT } else { theme::SURFACE_RAISED })
                            .hover(|s| s.bg(theme::ACCENT_HOVER))
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .text_color(theme::TEXT_PRIMARY)
                            .text_size(px(theme::FONT_SIZE_SM))
                            .on_click(cx.listener(move |this, _ev, _window, cx| {
                                this.select_as_base(id, cx);
                            }))
                            .child("Base"),
                    )
                    .child(
                        // New button
                        div()
                            .id(("btn-new", id.as_u128() as usize))
                            .px(px(theme::SPACING_XS))
                            .py(px(2.0))
                            .bg(if is_new { theme::ACCENT } else { theme::SURFACE_RAISED })
                            .hover(|s| s.bg(theme::ACCENT_HOVER))
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .text_color(theme::TEXT_PRIMARY)
                            .text_size(px(theme::FONT_SIZE_SM))
                            .on_click(cx.listener(move |this, _ev, _window, cx| {
                                this.select_as_new(id, cx);
                            }))
                            .child("New"),
                    )
                    .child(
                        // Delete button
                        div()
                            .id(("btn-delete", id.as_u128() as usize))
                            .px(px(theme::SPACING_XS))
                            .py(px(2.0))
                            .bg(theme::SURFACE_RAISED)
                            .hover(|s| s.bg(theme::COLOR_DELETED))
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .text_color(theme::TEXT_SECONDARY)
                            .text_size(px(theme::FONT_SIZE_SM))
                            .on_click(cx.listener(move |this, _ev, _window, cx| {
                                this.handle_delete(id, cx);
                            }))
                            .child("Delete"),
                    ),
            )
            .into_any_element()
    }

    fn render_action_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let can_compare = self.can_compare();

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_end()
            .px(px(theme::SPACING_SM))
            .py(px(theme::SPACING_XS))
            .bg(theme::SURFACE)
            .border_t_1()
            .border_color(theme::BORDER)
            .child(
                div()
                    .id("btn-compare")
                    .px(px(theme::SPACING_MD))
                    .py(px(theme::SPACING_XS))
                    .bg(if can_compare {
                        theme::ACCENT
                    } else {
                        theme::SURFACE_RAISED
                    })
                    .hover(|s| {
                        if can_compare {
                            s.bg(theme::ACCENT_HOVER)
                        } else {
                            s
                        }
                    })
                    .rounded(px(4.0))
                    .cursor_pointer()
                    .text_color(if can_compare {
                        theme::TEXT_PRIMARY
                    } else {
                        theme::TEXT_MUTED
                    })
                    .text_size(px(theme::FONT_SIZE_MD))
                    .on_click(cx.listener(|this, _ev, _window, cx| {
                        this.handle_compare(cx);
                    }))
                    .child("Compare"),
            )
    }
}

// ---------------------------------------------------------------------------
// Focusable
// ---------------------------------------------------------------------------

impl Focusable for ScanHistory {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------

impl Render for ScanHistory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows: Vec<AnyElement> = self
            .snapshots
            .iter()
            .map(|s| self.render_snapshot_row(s, cx))
            .collect();

        let action_bar = self.render_action_bar(cx);

        div()
            .flex()
            .flex_col()
            .w(px(theme::HISTORY_PANEL_WIDTH))
            .h_full()
            .bg(theme::HISTORY_BG)
            .border_r_1()
            .border_color(theme::BORDER)
            .track_focus(&self.focus_handle)
            .child(
                // Header
                div()
                    .px(px(theme::SPACING_SM))
                    .py(px(theme::SPACING_MD))
                    .bg(theme::SURFACE)
                    .border_b_1()
                    .border_color(theme::BORDER)
                    .text_color(theme::TEXT_PRIMARY)
                    .text_size(px(theme::FONT_SIZE_LG))
                    .child("Scan History"),
            )
            .child(
                // Scrollable list
                div()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(rows),
            )
            .child(action_bar)
    }
}