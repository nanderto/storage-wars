//! Database component — SQLite persistence layer using rusqlite.
//!
//! # Schema
//! - `scans`  — one row per completed scan (metadata)
//! - `nodes`  — one row per file-system node belonging to a scan
//!
//! Foreign key CASCADE ensures that deleting a scan removes all its nodes.
//! Indexes on `nodes(scan_id)` and `nodes(path)` keep queries fast.
//!
//! # Public API
//! - [`open_db`]              — open (or create + migrate) the on-disk database
//! - [`open_in_memory`]       — open a fresh in-memory database (testing)
//! - [`save_scan`]            — bulk-insert a scan tree in a single transaction
//! - [`load_scan_tree`]       — return a flat `Vec<DbNode>` for a given scan
//! - [`get_scans_for_drive`]  — return ordered `Vec<ScanMeta>` for a drive root
//! - [`delete_scan`]          — CASCADE-delete a scan and all its nodes

pub mod db;
pub mod error;
pub mod models;

pub use db::{delete_scan, get_scans_for_drive, load_scan_tree, open_db, open_in_memory, save_scan};
pub use error::DbError;
pub use models::{DbNode, ScanMeta, ScanNode};