//! # Database
//!
//! SQLite persistence layer using rusqlite.
//!
//! ## Schema
//! - `scans` table: top-level scan metadata (drive, timestamp, root path)
//! - `nodes` table: file/directory entries linked to a scan via foreign key CASCADE
//!
//! ## Public API
//! - [`open_db`]          – open (or create + migrate) the production database in APPDATA
//! - [`open_in_memory`]   – open a fresh in-memory database (testing)
//! - [`save_scan`]        – bulk-insert a scan tree in a single transaction
//! - [`load_scan_tree`]   – retrieve a flat list of [`DbNode`] for a given scan
//! - [`get_scans_for_drive`] – retrieve ordered [`ScanMeta`] list for a drive letter
//! - [`delete_scan`]      – CASCADE-delete a scan and all its nodes

pub mod db;
pub mod error;
pub mod models;
pub mod schema;

pub use db::{delete_scan, get_scans_for_drive, load_scan_tree, open_db, open_in_memory, save_scan};
pub use error::DbError;
pub use models::{DbNode, ScanMeta};