//! DDL statements and migration logic.

use rusqlite::Connection;

use crate::error::{DbError, DbResult};

/// SQL used to create the `scans` table.
const CREATE_SCANS: &str = "
CREATE TABLE IF NOT EXISTS scans (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    drive      TEXT    NOT NULL,
    root_path  TEXT    NOT NULL,
    created_at INTEGER NOT NULL,
    node_count INTEGER NOT NULL DEFAULT 0
);
";

/// SQL used to create the `nodes` table with CASCADE delete.
const CREATE_NODES: &str = "
CREATE TABLE IF NOT EXISTS nodes (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id    INTEGER NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
    path       TEXT    NOT NULL,
    is_dir     INTEGER NOT NULL DEFAULT 0,
    size_bytes INTEGER NOT NULL DEFAULT 0,
    parent_id  INTEGER REFERENCES nodes(id),
    depth      INTEGER NOT NULL DEFAULT 0
);
";

/// Index on `nodes(scan_id)` for fast per-scan lookups.
const CREATE_IDX_NODES_SCAN_ID: &str = "
CREATE INDEX IF NOT EXISTS idx_nodes_scan_id ON nodes(scan_id);
";

/// Index on `nodes(path)` for path-based lookups.
const CREATE_IDX_NODES_PATH: &str = "
CREATE INDEX IF NOT EXISTS idx_nodes_path ON nodes(path);
";

/// Apply all DDL migrations to `conn`.
///
/// This function is idempotent — it uses `IF NOT EXISTS` guards so it is
/// safe to call on an already-initialised database.
pub fn run_migrations(conn: &Connection) -> DbResult<()> {
    // Enable WAL mode for better concurrent read performance.
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;

    // Enforce foreign-key constraints (disabled by default in SQLite).
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    conn.execute_batch(CREATE_SCANS)
        .map_err(|e| DbError::MigrationFailed(format!("create scans table: {e}")))?;

    conn.execute_batch(CREATE_NODES)
        .map_err(|e| DbError::MigrationFailed(format!("create nodes table: {e}")))?;

    conn.execute_batch(CREATE_IDX_NODES_SCAN_ID)
        .map_err(|e| DbError::MigrationFailed(format!("create idx_nodes_scan_id: {e}")))?;

    conn.execute_batch(CREATE_IDX_NODES_PATH)
        .map_err(|e| DbError::MigrationFailed(format!("create idx_nodes_path: {e}")))?;

    Ok(())
}