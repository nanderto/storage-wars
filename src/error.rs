//! Unified error type for the database component.

use thiserror::Error;

/// All errors that can be produced by the database component.
#[derive(Debug, Error)]
pub enum DbError {
    /// Wraps any rusqlite error.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Raised when the APPDATA / home directory cannot be determined.
    #[error("Could not determine application data directory")]
    AppDataNotFound,

    /// Raised when a requested scan does not exist.
    #[error("Scan not found: id={0}")]
    ScanNotFound(i64),

    /// Any other I/O-level error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}