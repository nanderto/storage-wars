//! Core data model types for the disk usage analyzer desktop application.
//!
//! This crate provides:
//! - [`FsNode`]: Filesystem tree node with name, path, size, counts, and timestamps
//! - [`DbNode`]: Flat database representation with parent references
//! - [`ScanMeta`]: Scan session metadata
//! - [`DriveInfo`]: Drive name, volume label, and space information
//! - [`UiNode`]: FsNode wrapper with UI state (depth, expanded, scan progress)
//! - [`SizeChange`]: Delta classification with hex color codes
//! - [`ScanMessage`]: Enum for scan event messages

pub mod db_node;
pub mod drive_info;
pub mod fs_node;
pub mod scan_message;
pub mod scan_meta;
pub mod size_change;
pub mod ui_node;

pub use db_node::DbNode;
pub use drive_info::DriveInfo;
pub use fs_node::FsNode;
pub use scan_message::ScanMessage;
pub use scan_meta::ScanMeta;
pub use size_change::{SizeChange, SizeChangeTrend};
pub use ui_node::UiNode;