//! GPUI view components for disk space analyzer.
//!
//! This crate provides the following view components:
//! - [`AppView`]: Root orchestrator for drive selection, scanning, tree rendering,
//!   scan history, drive info panel, and custom title bar with window controls.
//! - [`DriveSelector`]: A `Select` widget with `Focusable` trait that formats
//!   drive labels with optional volume label and space information.
//! - [`TreeView`]: Hierarchical file list with columns (Name, % Parent, Size,
//!   Prev Size, % Prev, Files, Folders, Modified), chevrons, icons, indentation
//!   (16px per depth level), and progress bars with `SizeChange` colors.
//! - [`ScanHistory`]: A 280px-wide focusable panel showing scan list with
//!   Base/New selection, Compare and Delete buttons.

pub mod app_view;
pub mod drive_selector;
pub mod scan_history;
pub mod theme;
pub mod tree_view;

pub use app_view::AppView;
pub use drive_selector::DriveSelector;
pub use scan_history::ScanHistory;
pub use tree_view::TreeView;