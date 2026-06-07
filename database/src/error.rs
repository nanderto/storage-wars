//! Error types for the database crate.

use thiserror::Error;

/// All errors that can be produced by this crate.
#[derive(Debug, Error)]
pub enum DbError {
    /// Wraps any rusqlite error.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// The `APPDATA` / home directory could not be determined at runtime.
    #[error("Could not determine application data directory")]
    AppDataNotFound,

    /// A required scan was not found in the database.
    #[error("Scan not found: id={0}")]
    ScanNotFound(i64),

    /// Generic I/O error (e.g. creating the database directory).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, DbError>;