//! DDL statements and migration logic.

/// The single SQL script that creates all tables and indexes.
///
/// Designed to be idempotent (`IF NOT EXISTS`) so it can be re-run safely.
pub const MIGRATION_V1: &str = "
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS scans (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    drive_root  TEXT    NOT NULL,
    scanned_at  INTEGER NOT NULL,
    node_count  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS nodes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id     INTEGER NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
    parent_id   INTEGER REFERENCES nodes(id) ON DELETE CASCADE,
    path        TEXT    NOT NULL,
    is_dir      INTEGER NOT NULL DEFAULT 0,
    size        INTEGER NOT NULL DEFAULT 0,
    modified    INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_nodes_scan_id ON nodes(scan_id);
CREATE INDEX IF NOT EXISTS idx_nodes_path    ON nodes(path);
";

/// Applies all pending migrations to an open connection.
///
/// Currently there is only one migration (V1).  Future migrations should be
/// appended here and guarded by a `user_version` PRAGMA check.
pub fn run_migrations(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    conn.execute_batch(MIGRATION_V1)
}