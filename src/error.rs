//! Unified error type for the database component.

use thiserror::Error;

/// All errors that can be produced by the database component.
#[derive(Debug, Error)]
pub enum DbError {
    /// A rusqlite operation failed.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// The application-data directory could not be determined.
    #[error("could not determine application data directory")]
    AppDataNotFound,

    /// A requested scan does not exist.
    #[error("scan not found: id={0}")]
    ScanNotFound(i64),

    /// Generic I/O error (e.g. creating the database directory).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}