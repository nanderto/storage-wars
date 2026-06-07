use thiserror::Error;

/// All errors that can be produced by the database layer.
#[derive(Debug, Error)]
pub enum DbError {
    /// Wraps any rusqlite error.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Raised when the APPDATA environment variable is not set or cannot be resolved.
    #[error("Could not determine application data directory: {0}")]
    AppDataDir(String),

    /// Raised when a required record is not found.
    #[error("Record not found: {0}")]
    NotFound(String),
}