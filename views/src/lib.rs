//! GPUI view components for disk space analyzer.
//!
//! This crate provides the following view components:
//! - [`AppView`]: Root orchestrator view managing drive selection, scanning,
//!   tree rendering, scan history, and drive info panel.
//! - [`DriveSelector`]: A focusable `Select`-style widget for choosing drives,
//!   formatting labels with optional volume label and free space.
//! - [`TreeView`]: Hierarchical file/folder list with columns (Name, % Parent,
//!   Size, Prev Size, % Prev, Files, Folders, Modified), chevrons, icons,
//!   indentation (16 px × depth), and progress bars with `SizeChange` colors.
//! - [`ScanHistory`]: A 280 px-wide focusable panel listing scans with
//!   Base/New selection, Compare, and Delete actions.

pub mod app_view;
pub mod drive_selector;
pub mod scan_history;
pub mod theme;
pub mod tree_view;

pub use app_view::AppView;
pub use drive_selector::DriveSelector;
pub use scan_history::ScanHistory;
pub use tree_view::TreeView;