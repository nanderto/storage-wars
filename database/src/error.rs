//! Error types for the database component.

use thiserror::Error;

/// All errors that can be produced by the database component.
#[derive(Debug, Error)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Environment variable not found: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database path is invalid: {0}")]
    InvalidPath(String),

    #[error("Scan not found: id={0}")]
    ScanNotFound(i64),

    #[error("Migration failed: {0}")]
    MigrationFailed(String),
}

pub type DbResult<T> = Result<T, DbError>;