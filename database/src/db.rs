use std::path::PathBuf;

use rusqlite::{params, Connection};

use crate::error::DbError;
use crate::models::{DbNode, ScanMeta};
use crate::schema::CREATE_SCHEMA;

// ─── Connection helpers ───────────────────────────────────────────────────────

/// Open (or create) the production SQLite database located at
/// `%APPDATA%\disk_scanner\database.db` on Windows, or
/// `$HOME/.local/share/disk_scanner/database.db` on Unix-likes.
///
/// Runs the schema migration on every open (idempotent `CREATE IF NOT EXISTS`).
pub fn open_db() -> Result<Connection, DbError> {
    let path = app_data_path()?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DbError::AppDataDir(e.to_string()))?;
    }

    let conn = Connection::open(&path)?;
    apply_schema(&conn)?;
    Ok(conn)
}

/// Open a fresh **in-memory** SQLite database and apply the schema.
/// Intended for unit / integration tests.
pub fn open_in_memory() -> Result<Connection, DbError> {
    let conn = Connection::open_in_memory()?;
    apply_schema(&conn)?;
    Ok(conn)
}

// ─── Write operations ─────────────────────────────────────────────────────────

/// Persist a complete scan tree in a single transaction.
///
/// `nodes` must be ordered **depth-first** so that every parent node appears
/// before its children (the function assigns `parent_id` based on the
/// `parent_id` field already set on each [`DbNode`]).
///
/// Returns the newly created `scan_id`.
pub fn save_scan(
    conn: &Connection,
    drive: &str,
    root_path: &str,
    scanned_at: i64,
    total_bytes: i64,
    nodes: &[DbNode],
) -> Result<i64, DbError> {
    // Enable foreign keys for this connection (WAL pragma is set in schema).
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let tx = conn.unchecked_transaction()?;

    tx.execute(
        "INSERT INTO scans (drive, root_path, scanned_at, total_bytes)
         VALUES (?1, ?2, ?3, ?4)",
        params![drive, root_path, scanned_at, total_bytes],
    )?;

    let scan_id = tx.last_insert_rowid();

    {
        let mut stmt = tx.prepare(
            "INSERT INTO nodes (scan_id, path, is_dir, size_bytes, parent_id, depth)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        for node in nodes {
            stmt.execute(params![
                scan_id,
                node.path,
                node.is_dir as i64,
                node.size_bytes,
                node.parent_id,
                node.depth,
            ])?;
        }
    }

    tx.commit()?;
    Ok(scan_id)
}

/// Delete a scan and all its nodes (CASCADE handled by SQLite foreign key).
pub fn delete_scan(conn: &Connection, scan_id: i64) -> Result<(), DbError> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    let affected = conn.execute("DELETE FROM scans WHERE id = ?1", params![scan_id])?;
    if affected == 0 {
        return Err(DbError::NotFound(format!("scan id {scan_id}")));
    }
    Ok(())
}

// ─── Read operations ──────────────────────────────────────────────────────────

/// Load all nodes for a given `scan_id` as a flat list ordered by `depth`,
/// then by `path` within the same depth level.
pub fn load_scan_tree(conn: &Connection, scan_id: i64) -> Result<Vec<DbNode>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, scan_id, path, is_dir, size_bytes, parent_id, depth
         FROM   nodes
         WHERE  scan_id = ?1
         ORDER  BY depth ASC, path ASC",
    )?;

    let nodes = stmt
        .query_map(params![scan_id], row_to_db_node)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(nodes)
}

/// Return all scans for a given drive letter / mount point, ordered by
/// `scanned_at` descending (most recent first).
pub fn get_scans_for_drive(conn: &Connection, drive: &str) -> Result<Vec<ScanMeta>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, drive, root_path, scanned_at, total_bytes
         FROM   scans
         WHERE  drive = ?1
         ORDER  BY scanned_at DESC",
    )?;

    let scans = stmt
        .query_map(params![drive], row_to_scan_meta)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(scans)
}

// ─── Private helpers ──────────────────────────────────────────────────────────

fn apply_schema(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(CREATE_SCHEMA)?;
    Ok(())
}

fn row_to_db_node(row: &rusqlite::Row<'_>) -> rusqlite::Result<DbNode> {
    Ok(DbNode {
        id: row.get(0)?,
        scan_id: row.get(1)?,
        path: row.get(2)?,
        is_dir: row.get::<_, i64>(3)? != 0,
        size_bytes: row.get(4)?,
        parent_id: row.get(5)?,
        depth: row.get(6)?,
    })
}

fn row_to_scan_meta(row: &rusqlite::Row<'_>) -> rusqlite::Result<ScanMeta> {
    Ok(ScanMeta {
        id: row.get(0)?,
        drive: row.get(1)?,
        root_path: row.get(2)?,
        scanned_at: row.get(3)?,
        total_bytes: row.get(4)?,
    })
}

/// Resolve the platform-specific application data directory.
fn app_data_path() -> Result<PathBuf, DbError> {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var("APPDATA")
            .map_err(|_| DbError::AppDataDir("APPDATA env var not set".into()))?;
        Ok(PathBuf::from(base)
            .join("disk_scanner")
            .join("database.db"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        let base = std::env::var("HOME")
            .map_err(|_| DbError::AppDataDir("HOME env var not set".into()))?;
        Ok(PathBuf::from(base)
            .join(".local")
            .join("share")
            .join("disk_scanner")
            .join("database.db"))
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_nodes(scan_id: i64) -> Vec<DbNode> {
        vec![
            DbNode {
                id: 0,
                scan_id,
                path: "/tmp/root".into(),
                is_dir: true,
                size_bytes: 0,
                parent_id: None,
                depth: 0,
            },
            DbNode {
                id: 0,
                scan_id,
                path: "/tmp/root/file.txt".into(),
                is_dir: false,
                size_bytes: 1024,
                parent_id: None, // will be resolved by caller in real usage
                depth: 1,
            },
        ]
    }

    #[test]
    fn test_open_in_memory() {
        let conn = open_in_memory().expect("in-memory db should open");
        // Verify schema tables exist.
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('scans','nodes')",
                [],
                |r| r.get(0),
            )
            .expect("query should succeed");
        assert_eq!(count, 2, "both tables must exist after schema migration");
    }

    #[test]
    fn test_save_and_load_scan() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes(0 /* placeholder, overwritten by save_scan */);

        let scan_id = save_scan(&conn, "C", "C:\\", 1_700_000_000, 1024, &nodes)
            .expect("save_scan should succeed");

        assert!(scan_id > 0);

        let loaded = load_scan_tree(&conn, scan_id).expect("load_scan_tree should succeed");
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].path, "/tmp/root");
        assert_eq!(loaded[1].path, "/tmp/root/file.txt");
    }

    #[test]
    fn test_get_scans_for_drive() {
        let conn = open_in_memory().unwrap();
        let nodes: Vec<DbNode> = vec![];

        save_scan(&conn, "D", "D:\\", 1_700_000_001, 0, &nodes).unwrap();
        save_scan(&conn, "D", "D:\\", 1_700_000_002, 0, &nodes).unwrap();
        save_scan(&conn, "C", "C:\\", 1_700_000_003, 0, &nodes).unwrap();

        let d_scans = get_scans_for_drive(&conn, "D").unwrap();
        assert_eq!(d_scans.len(), 2);
        // Most recent first.
        assert!(d_scans[0].scanned_at > d_scans[1].scanned_at);

        let c_scans = get_scans_for_drive(&conn, "C").unwrap();
        assert_eq!(c_scans.len(), 1);
    }

    #[test]
    fn test_delete_scan_cascade() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes(0);

        let scan_id = save_scan(&conn, "E", "E:\\", 1_700_000_010, 2048, &nodes).unwrap();

        // Nodes must exist before deletion.
        let before = load_scan_tree(&conn, scan_id).unwrap();
        assert_eq!(before.len(), 2);

        delete_scan(&conn, scan_id).expect("delete_scan should succeed");

        // Nodes must be gone after CASCADE delete.
        let after = load_scan_tree(&conn, scan_id).unwrap();
        assert!(after.is_empty(), "nodes should be cascade-deleted");
    }

    #[test]
    fn test_delete_nonexistent_scan_returns_error() {
        let conn = open_in_memory().unwrap();
        let result = delete_scan(&conn, 9999);
        assert!(
            matches!(result, Err(DbError::NotFound(_))),
            "expected NotFound error"
        );
    }

    #[test]
    fn test_indexes_exist() {
        let conn = open_in_memory().unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='index'
                   AND name IN ('idx_nodes_scan_id','idx_nodes_path')",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 2, "both indexes must be created by schema migration");
    }
}