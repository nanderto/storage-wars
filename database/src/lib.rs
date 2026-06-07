//! SQLite persistence layer using rusqlite.
//!
//! Provides schema management, scan storage, and retrieval functions
//! for the disk scanner desktop application.

pub mod db;
pub mod error;
pub mod models;
pub mod schema;

pub use db::{open_db, open_in_memory};
pub use error::DbError;
pub use models::{DbNode, ScanMeta};