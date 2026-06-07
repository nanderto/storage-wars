//! GPUI view components for disk space analyzer.
//!
//! # Components
//!
//! - [`AppView`] — Root orchestrator: drive selection, scanning, tree rendering,
//!   scan history, drive info panel, and custom title bar with window controls.
//! - [`DriveSelector`] — A `Select`-style widget implementing `Focusable`,
//!   formats drive labels with optional volume label and available space.
//! - [`TreeView`] — Hierarchical file list with columns (Name, % Parent, Size,
//!   Prev Size, % Prev, Files, Folders, Modified), chevrons, icons, indentation
//!   at 16 px per depth level, and progress bars with `SizeChange` colours.
//! - [`ScanHistory`] — 280 px-wide focusable panel showing scan list with
//!   Base/New selection, Compare and Delete buttons.

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