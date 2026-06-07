//! Core database functions: open, save, load, query, delete.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

use crate::error::{DbError, DbResult};
use crate::models::{DbNode, ScanMeta};
use crate::schema::run_migrations;

// ─── Connection helpers ───────────────────────────────────────────────────────

/// Open (or create) the application database stored under `%APPDATA%` on
/// Windows or `$HOME/.local/share` on other platforms.
///
/// Runs all pending migrations before returning.
pub fn open_db() -> DbResult<Connection> {
    let path = app_db_path()?;

    // Ensure the parent directory exists.
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(&path)?;
    configure_connection(&conn)?;
    run_migrations(&conn)?;
    Ok(conn)
}

/// Open an in-memory database suitable for unit tests.
///
/// Runs all pending migrations before returning.
pub fn open_in_memory() -> DbResult<Connection> {
    let conn = Connection::open_in_memory()?;
    configure_connection(&conn)?;
    run_migrations(&conn)?;
    Ok(conn)
}

/// Apply runtime PRAGMAs that must be set on every new connection.
fn configure_connection(conn: &Connection) -> DbResult<()> {
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;
    Ok(())
}

/// Resolve the platform-appropriate path for the application database file.
fn app_db_path() -> DbResult<PathBuf> {
    #[cfg(target_os = "windows")]
    let base = {
        let appdata = std::env::var("APPDATA")?;
        PathBuf::from(appdata)
    };

    #[cfg(not(target_os = "windows"))]
    let base = {
        let home = std::env::var("HOME")?;
        PathBuf::from(home).join(".local").join("share")
    };

    Ok(base.join("disk-scanner").join("database.db"))
}

// ─── Write operations ─────────────────────────────────────────────────────────

/// A node supplied by the caller for bulk insertion.
#[derive(Debug, Clone)]
pub struct InputNode {
    pub path: String,
    pub is_dir: bool,
    pub size_bytes: i64,
    /// Index into the caller's slice that represents this node's parent,
    /// or `None` for the root.
    pub parent_index: Option<usize>,
    pub depth: i32,
}

/// Persist a complete scan in a single transaction using depth-first bulk
/// insert order.
///
/// Returns the newly created `scan_id`.
pub fn save_scan(
    conn: &Connection,
    drive: &str,
    root_path: &str,
    nodes: &[InputNode],
) -> DbResult<i64> {
    let created_at = unix_now();
    let node_count = nodes.len() as i64;

    let tx = conn.unchecked_transaction()?;

    // Insert the scan header row.
    tx.execute(
        "INSERT INTO scans (drive, root_path, created_at, node_count)
         VALUES (?1, ?2, ?3, ?4)",
        params![drive, root_path, created_at, node_count],
    )?;
    let scan_id = tx.last_insert_rowid();

    // Pre-allocate a mapping from caller index → database row id.
    let mut db_ids: Vec<i64> = vec![0i64; nodes.len()];

    let mut stmt = tx.prepare(
        "INSERT INTO nodes (scan_id, path, is_dir, size_bytes, parent_id, depth)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;

    for (idx, node) in nodes.iter().enumerate() {
        let parent_db_id: Option<i64> = node
            .parent_index
            .map(|pi| db_ids[pi]);

        stmt.execute(params![
            scan_id,
            node.path,
            node.is_dir as i32,
            node.size_bytes,
            parent_db_id,
            node.depth,
        ])?;

        db_ids[idx] = tx.last_insert_rowid();
    }

    drop(stmt);
    tx.commit()?;

    Ok(scan_id)
}

// ─── Read operations ──────────────────────────────────────────────────────────

/// Load all nodes for a given `scan_id` as a flat list ordered by `depth`
/// then `path`.
pub fn load_scan_tree(conn: &Connection, scan_id: i64) -> DbResult<Vec<DbNode>> {
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

/// Return all scans recorded for a given drive letter / mount-point, ordered
/// by `created_at` descending (most recent first).
pub fn get_scans_for_drive(conn: &Connection, drive: &str) -> DbResult<Vec<ScanMeta>> {
    let mut stmt = conn.prepare(
        "SELECT id, drive, root_path, created_at, node_count
         FROM   scans
         WHERE  drive = ?1
         ORDER  BY created_at DESC",
    )?;

    let rows = stmt.query_map(params![drive], |row| {
        Ok(ScanMeta {
            id: row.get(0)?,
            drive: row.get(1)?,
            root_path: row.get(2)?,
            created_at: row.get(3)?,
            node_count: row.get(4)?,
        })
    })?;

    rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
}

/// Delete a scan and all its nodes (CASCADE handles the nodes table).
pub fn delete_scan(conn: &Connection, scan_id: i64) -> DbResult<()> {
    let affected = conn.execute("DELETE FROM scans WHERE id = ?1", params![scan_id])?;

    if affected == 0 {
        return Err(DbError::ScanNotFound(scan_id));
    }

    Ok(())
}

// ─── Utilities ────────────────────────────────────────────────────────────────

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_nodes() -> Vec<InputNode> {
        vec![
            InputNode {
                path: "/tmp/scan".into(),
                is_dir: true,
                size_bytes: 0,
                parent_index: None,
                depth: 0,
            },
            InputNode {
                path: "/tmp/scan/file_a.txt".into(),
                is_dir: false,
                size_bytes: 1024,
                parent_index: Some(0),
                depth: 1,
            },
            InputNode {
                path: "/tmp/scan/subdir".into(),
                is_dir: true,
                size_bytes: 0,
                parent_index: Some(0),
                depth: 1,
            },
            InputNode {
                path: "/tmp/scan/subdir/file_b.txt".into(),
                is_dir: false,
                size_bytes: 2048,
                parent_index: Some(2),
                depth: 2,
            },
        ]
    }

    #[test]
    fn open_in_memory_succeeds() {
        open_in_memory().expect("in-memory db should open");
    }

    #[test]
    fn save_and_load_scan() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes();

        let scan_id = save_scan(&conn, "C:\\", "/tmp/scan", &nodes).unwrap();
        assert!(scan_id > 0);

        let loaded = load_scan_tree(&conn, scan_id).unwrap();
        assert_eq!(loaded.len(), nodes.len());

        // Root node should be at depth 0.
        assert!(loaded.iter().any(|n| n.depth == 0 && n.is_dir));
    }

    #[test]
    fn get_scans_for_drive_returns_ordered() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes();

        let _id1 = save_scan(&conn, "C:\\", "/tmp/scan1", &nodes).unwrap();
        let _id2 = save_scan(&conn, "C:\\", "/tmp/scan2", &nodes).unwrap();
        let _id3 = save_scan(&conn, "D:\\", "/mnt/scan3", &nodes).unwrap();

        let c_scans = get_scans_for_drive(&conn, "C:\\").unwrap();
        assert_eq!(c_scans.len(), 2);

        let d_scans = get_scans_for_drive(&conn, "D:\\").unwrap();
        assert_eq!(d_scans.len(), 1);

        // Most recent first.
        assert!(c_scans[0].created_at >= c_scans[1].created_at);
    }

    #[test]
    fn delete_scan_removes_nodes_via_cascade() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes();

        let scan_id = save_scan(&conn, "C:\\", "/tmp/scan", &nodes).unwrap();

        // Nodes exist before deletion.
        let before = load_scan_tree(&conn, scan_id).unwrap();
        assert!(!before.is_empty());

        delete_scan(&conn, scan_id).unwrap();

        // Nodes should be gone (CASCADE).
        let after = load_scan_tree(&conn, scan_id).unwrap();
        assert!(after.is_empty());
    }

    #[test]
    fn delete_nonexistent_scan_returns_error() {
        let conn = open_in_memory().unwrap();
        let result = delete_scan(&conn, 9999);
        assert!(matches!(result, Err(DbError::ScanNotFound(9999))));
    }

    #[test]
    fn node_parent_ids_are_set_correctly() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes();

        let scan_id = save_scan(&conn, "C:\\", "/tmp/scan", &nodes).unwrap();
        let loaded = load_scan_tree(&conn, scan_id).unwrap();

        // Root has no parent.
        let root = loaded.iter().find(|n| n.depth == 0).unwrap();
        assert!(root.parent_id.is_none());

        // Depth-1 nodes have the root as parent.
        let depth1: Vec<_> = loaded.iter().filter(|n| n.depth == 1).collect();
        assert!(!depth1.is_empty());
        for node in &depth1 {
            assert_eq!(node.parent_id, Some(root.id));
        }
    }
}