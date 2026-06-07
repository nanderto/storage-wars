/// DDL executed once when the database is first opened (migration v1).
pub const CREATE_SCHEMA: &str = "
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS scans (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    drive       TEXT    NOT NULL,
    root_path   TEXT    NOT NULL,
    scanned_at  INTEGER NOT NULL,
    total_bytes INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS nodes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id     INTEGER NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
    path        TEXT    NOT NULL,
    is_dir      INTEGER NOT NULL DEFAULT 0,
    size_bytes  INTEGER NOT NULL DEFAULT 0,
    parent_id   INTEGER,
    depth       INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_nodes_scan_id ON nodes(scan_id);
CREATE INDEX IF NOT EXISTS idx_nodes_path    ON nodes(path);
";