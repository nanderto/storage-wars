//! Core database operations.

use std::collections::HashMap;
use std::path::PathBuf;

use rusqlite::{params, Connection};

use crate::error::DbError;
use crate::models::{DbNode, ScanMeta, ScanNode};

// ─── Schema ──────────────────────────────────────────────────────────────────

const SCHEMA_SQL: &str = r#"
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS scans (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    drive_root  TEXT    NOT NULL,
    scanned_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    node_count  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS nodes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id     INTEGER NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
    path        TEXT    NOT NULL,
    is_dir      INTEGER NOT NULL DEFAULT 0,
    size_bytes  INTEGER NOT NULL DEFAULT 0,
    parent_id   INTEGER REFERENCES nodes(id) ON DELETE CASCADE,
    depth       INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_nodes_scan_id ON nodes(scan_id);
CREATE INDEX IF NOT EXISTS idx_nodes_path    ON nodes(path);
"#;

// ─── Connection helpers ───────────────────────────────────────────────────────

/// Open (or create) the on-disk SQLite database, applying the schema migration.
///
/// The database file is placed in:
/// - **Windows** — `%APPDATA%\disk-scanner\database.db`
/// - **macOS**   — `~/Library/Application Support/disk-scanner/database.db`
/// - **Linux**   — `$XDG_DATA_HOME/disk-scanner/database.db`
///   (falls back to `~/.local/share/disk-scanner/database.db`)
pub fn open_db() -> Result<Connection, DbError> {
    let path = app_db_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(&path)?;
    migrate(&conn)?;
    Ok(conn)
}

/// Open a fresh **in-memory** database with the schema applied.
///
/// Intended for unit / integration tests.
pub fn open_in_memory() -> Result<Connection, DbError> {
    let conn = Connection::open_in_memory()?;
    migrate(&conn)?;
    Ok(conn)
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Bulk-insert a complete scan tree inside a single transaction.
///
/// `nodes` must be ordered **depth-first** (parents before children) so that
/// parent database IDs are available when child rows are inserted.
///
/// Returns the newly created `scan_id`.
pub fn save_scan(
    conn: &Connection,
    drive_root: &str,
    nodes: &[ScanNode],
) -> Result<i64, DbError> {
    // Enable foreign keys for this connection (WAL + FK are set per-connection).
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let scan_id: i64 = conn.query_row(
        "INSERT INTO scans (drive_root, node_count) VALUES (?1, 0) RETURNING id",
        params![drive_root],
        |row| row.get(0),
    )?;

    // Map temp_id → real database id so children can reference their parent.
    let mut id_map: HashMap<i64, i64> = HashMap::with_capacity(nodes.len());

    {
        let tx = conn.unchecked_transaction()?;

        let mut stmt = tx.prepare(
            "INSERT INTO nodes (scan_id, path, is_dir, size_bytes, parent_id, depth)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        for node in nodes {
            let real_parent_id: Option<i64> = node
                .temp_parent_id
                .map(|tid| id_map.get(&tid).copied())
                .flatten();

            stmt.execute(params![
                scan_id,
                node.path,
                node.is_dir as i32,
                node.size_bytes,
                real_parent_id,
                node.depth,
            ])?;

            let real_id = tx.last_insert_rowid();
            id_map.insert(node.temp_id, real_id);
        }

        tx.commit()?;
    }

    // Update the node count on the scan row.
    conn.execute(
        "UPDATE scans SET node_count = ?1 WHERE id = ?2",
        params![nodes.len() as i64, scan_id],
    )?;

    Ok(scan_id)
}

/// Return a flat list of every [`DbNode`] that belongs to `scan_id`.
///
/// Rows are ordered by `depth ASC, path ASC`.
pub fn load_scan_tree(conn: &Connection, scan_id: i64) -> Result<Vec<DbNode>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, scan_id, path, is_dir, size_bytes, parent_id, depth
         FROM   nodes
         WHERE  scan_id = ?1
         ORDER  BY depth ASC, path ASC",
    )?;

    let rows = stmt.query_map(params![scan_id], |row| {
        Ok(DbNode {
            id: row.get(0)?,
            scan_id: row.get(1)?,
            path: row.get(2)?,
            is_dir: row.get::<_, i32>(3)? != 0,
            size_bytes: row.get(4)?,
            parent_id: row.get(5)?,
            depth: row.get(6)?,
        })
    })?;

    rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
}

/// Return all scans whose `drive_root` matches `drive_root`, ordered by
/// `scanned_at DESC` (most recent first).
pub fn get_scans_for_drive(
    conn: &Connection,
    drive_root: &str,
) -> Result<Vec<ScanMeta>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, drive_root, scanned_at, node_count
         FROM   scans
         WHERE  drive_root = ?1
         ORDER  BY scanned_at DESC",
    )?;

    let rows = stmt.query_map(params![drive_root], |row| {
        Ok(ScanMeta {
            id: row.get(0)?,
            drive_root: row.get(1)?,
            scanned_at: row.get(2)?,
            node_count: row.get(3)?,
        })
    })?;

    rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
}

/// Delete a scan and all its nodes (CASCADE handles the `nodes` rows).
///
/// Returns [`DbError::ScanNotFound`] if no scan with `scan_id` exists.
pub fn delete_scan(conn: &Connection, scan_id: i64) -> Result<(), DbError> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let affected = conn.execute("DELETE FROM scans WHERE id = ?1", params![scan_id])?;

    if affected == 0 {
        return Err(DbError::ScanNotFound(scan_id));
    }

    Ok(())
}

// ─── Private helpers ──────────────────────────────────────────────────────────

/// Apply the schema (idempotent — uses `CREATE … IF NOT EXISTS`).
fn migrate(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(SCHEMA_SQL)?;
    Ok(())
}

/// Resolve the platform-appropriate path for the database file.
fn app_db_path() -> Result<PathBuf, DbError> {
    // Prefer the standard OS data directory; fall back to the current directory.
    let base = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .or_else(|| {
            // macOS / Linux: $HOME/Library/Application Support or $XDG_DATA_HOME
            #[cfg(target_os = "macos")]
            {
                std::env::var_os("HOME").map(|h| {
                    PathBuf::from(h)
                        .join("Library")
                        .join("Application Support")
                })
            }
            #[cfg(not(any(target_os = "windows", target_os = "macos")))]
            {
                std::env::var_os("XDG_DATA_HOME")
                    .map(PathBuf::from)
                    .or_else(|| {
                        std::env::var_os("HOME")
                            .map(|h| PathBuf::from(h).join(".local").join("share"))
                    })
            }
            #[cfg(target_os = "windows")]
            {
                None
            }
        })
        .ok_or(DbError::AppDataNotFound)?;

    Ok(base.join("disk-scanner").join("database.db"))
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_nodes() -> Vec<ScanNode> {
        vec![
            ScanNode {
                temp_id: 1,
                temp_parent_id: None,
                path: "/".to_string(),
                is_dir: true,
                size_bytes: 0,
                depth: 0,
            },
            ScanNode {
                temp_id: 2,
                temp_parent_id: Some(1),
                path: "/home".to_string(),
                is_dir: true,
                size_bytes: 0,
                depth: 1,
            },
            ScanNode {
                temp_id: 3,
                temp_parent_id: Some(2),
                path: "/home/file.txt".to_string(),
                is_dir: false,
                size_bytes: 1024,
                depth: 2,
            },
        ]
    }

    #[test]
    fn test_open_in_memory() {
        open_in_memory().expect("in-memory db should open");
    }

    #[test]
    fn test_save_and_load_scan() {
        let conn = open_in_memory().unwrap();
        let scan_id = save_scan(&conn, "/", &sample_nodes()).unwrap();
        assert!(scan_id > 0);

        let nodes = load_scan_tree(&conn, scan_id).unwrap();
        assert_eq!(nodes.len(), 3);
        // Root node has no parent.
        let root = nodes.iter().find(|n| n.depth == 0).unwrap();
        assert!(root.parent_id.is_none());
        // Leaf node has correct size.
        let leaf = nodes.iter().find(|n| n.path == "/home/file.txt").unwrap();
        assert_eq!(leaf.size_bytes, 1024);
        assert!(!leaf.is_dir);
    }

    #[test]
    fn test_get_scans_for_drive() {
        let conn = open_in_memory().unwrap();
        save_scan(&conn, "C:\\", &sample_nodes()).unwrap();
        save_scan(&conn, "C:\\", &sample_nodes()).unwrap();
        save_scan(&conn, "D:\\", &sample_nodes()).unwrap();

        let c_scans = get_scans_for_drive(&conn, "C:\\").unwrap();
        assert_eq!(c_scans.len(), 2);

        let d_scans = get_scans_for_drive(&conn, "D:\\").unwrap();
        assert_eq!(d_scans.len(), 1);

        let none = get_scans_for_drive(&conn, "Z:\\").unwrap();
        assert!(none.is_empty());
    }

    #[test]
    fn test_delete_scan_cascades() {
        let conn = open_in_memory().unwrap();
        let scan_id = save_scan(&conn, "/", &sample_nodes()).unwrap();

        delete_scan(&conn, scan_id).unwrap();

        let nodes = load_scan_tree(&conn, scan_id).unwrap();
        assert!(nodes.is_empty(), "nodes should be cascade-deleted");
    }

    #[test]
    fn test_delete_nonexistent_scan() {
        let conn = open_in_memory().unwrap();
        let result = delete_scan(&conn, 9999);
        assert!(matches!(result, Err(DbError::ScanNotFound(9999))));
    }

    #[test]
    fn test_node_count_updated() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes();
        let scan_id = save_scan(&conn, "/", &nodes).unwrap();

        let scans = get_scans_for_drive(&conn, "/").unwrap();
        let meta = scans.iter().find(|s| s.id == scan_id).unwrap();
        assert_eq!(meta.node_count, nodes.len() as i64);
    }
}