//! Core data model types for the disk analyzer desktop application.
//!
//! This crate provides the fundamental data structures used across the
//! application for representing filesystem nodes, database records,
//! scan metadata, drive information, UI state, and scan messaging.

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