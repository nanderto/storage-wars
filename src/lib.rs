//! Database component – SQLite persistence layer.
//!
//! Provides:
//! - [`open_db`]          – open (or create + migrate) the production database
//! - [`open_in_memory`]   – open an in-memory database for testing
//! - [`save_scan`]        – depth-first bulk insert of a scan tree in a transaction
//! - [`load_scan_tree`]   – load a flat list of [`DbNode`]s for a given scan
//! - [`get_scans_for_drive`] – ordered list of [`ScanMeta`] for a drive root
//! - [`delete_scan`]      – CASCADE-delete a scan and all its nodes

pub mod db;
pub mod error;
pub mod models;

pub use db::{delete_scan, get_scans_for_drive, load_scan_tree, open_db, open_in_memory, save_scan};
pub use error::DbError;
pub use models::{DbNode, ScanMeta, ScanNode};