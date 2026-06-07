//! GPUI view components for disk space analyzer.
//!
//! # Components
//! - [`AppView`] — root orchestrator: drive selection, scanning, tree, history, info panel, title bar
//! - [`DriveSelector`] — focusable Select widget for drive selection
//! - [`TreeView`] — hierarchical file list with columns and progress bars
//! - [`ScanHistory`] — 280px-wide panel for scan list with Base/New selection

pub mod app_view;
pub mod drive_selector;
pub mod scan_history;
pub mod theme;
pub mod tree_view;
pub mod types;

pub use app_view::AppView;
pub use drive_selector::DriveSelector;
pub use scan_history::ScanHistory;
pub use tree_view::TreeView;