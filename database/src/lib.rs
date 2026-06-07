//! # database
//!
//! SQLite persistence layer for the disk scanner application.
//!
//! ## Overview
//!
//! This crate provides:
//! - [`open_db`] — opens (or creates) the production database under `APPDATA`,
//!   running all pending migrations automatically.
//! - [`open_in_memory`] — opens a fully-migrated in-memory database for tests.
//! - [`save_scan`] — depth-first bulk insert of a scan tree inside a transaction.
//! - [`load_scan_tree`] — returns a flat `Vec<DbNode>` for a given scan.
//! - [`get_scans_for_drive`] — returns ordered `Vec<ScanMeta>` for a drive root.
//! - [`delete_scan`] — CASCADE-deletes a scan and all its nodes.

pub mod db;
pub mod error;
pub mod models;
pub mod schema;

pub use db::{delete_scan, get_scans_for_drive, load_scan_tree, open_db, open_in_memory, save_scan};
pub use error::DbError;
pub use models::{DbNode, ScanMeta};