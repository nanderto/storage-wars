//! Public API — connection helpers and CRUD operations.

use rusqlite::{params, Connection};

use crate::error::{DbError, Result};
use crate::models::{DbNode, ScanMeta};
use crate::schema::run_migrations;

// ─── Connection helpers ───────────────────────────────────────────────────────

/// Opens (or creates) the production SQLite database.
///
/// The file is placed at:
/// - **Windows** — `%APPDATA%\DiskScanner\scanner.db`
/// - **macOS / Linux** — `$HOME/.local/share/DiskScanner/scanner.db`
///
/// All pending migrations are applied before the connection is returned.
pub fn open_db() -> Result<Connection> {
    let data_dir = get_app_data_dir()?;
    std::fs::create_dir_all(&data_dir)?;
    let db_path = data_dir.join("scanner.db");
    let conn = Connection::open(db_path)?;
    configure_connection(&conn)?;
    run_migrations(&conn)?;
    Ok(conn)
}

/// Opens a fully-migrated **in-memory** database.
///
/// Intended for unit and integration tests — no files are written to disk.
pub fn open_in_memory() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    configure_connection(&conn)?;
    run_migrations(&conn)?;
    Ok(conn)
}

/// Applies connection-level PRAGMAs that must be set on every new connection.
fn configure_connection(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA foreign_keys = ON;
         PRAGMA synchronous   = NORMAL;",
    )
}

/// Returns the platform-appropriate application-data directory.
fn get_app_data_dir() -> Result<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA")
            .map_err(|_| DbError::AppDataNotFound)?;
        Ok(std::path::PathBuf::from(appdata).join("DiskScanner"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        let home = std::env::var("HOME")
            .map_err(|_| DbError::AppDataNotFound)?;
        Ok(std::path::PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("DiskScanner"))
    }
}

// ─── Write operations ─────────────────────────────────────────────────────────

/// Inserts a complete scan tree into the database inside a single transaction.
///
/// `nodes` must be ordered **depth-first** (parents before children) so that
/// `parent_id` foreign-key references are satisfied during the bulk insert.
///
/// Returns the newly-assigned `scan_id`.
pub fn save_scan(
    conn: &Connection,
    drive_root: &str,
    scanned_at: i64,
    nodes: &[DbNode],
) -> Result<i64> {
    let tx = conn.unchecked_transaction()?;

    // Insert the scan header row first.
    tx.execute(
        "INSERT INTO scans (drive_root, scanned_at, node_count) VALUES (?1, ?2, ?3)",
        params![drive_root, scanned_at, nodes.len() as i64],
    )?;
    let scan_id = tx.last_insert_rowid();

    // Bulk-insert every node.  Because `nodes` is depth-first we can rely on
    // the fact that a parent's rowid is already known when we insert a child.
    // However, callers supply logical `parent_id` values that reference the
    // *index* within the slice (0-based), not the actual database rowid.
    // We therefore maintain a mapping from slice-index → rowid.
    let mut rowid_map: Vec<i64> = Vec::with_capacity(nodes.len());

    {
        let mut stmt = tx.prepare(
            "INSERT INTO nodes (scan_id, parent_id, path, is_dir, size, modified)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        for (idx, node) in nodes.iter().enumerate() {
            // Resolve the logical parent_id (slice index) to a real rowid.
            let db_parent_id: Option<i64> = node
                .parent_id
                .map(|pi| rowid_map[pi as usize]);

            stmt.execute(params![
                scan_id,
                db_parent_id,
                node.path,
                node.is_dir as i64,
                node.size,
                node.modified,
            ])?;

            let rowid = tx.last_insert_rowid();
            rowid_map.push(rowid);
            let _ = idx; // suppress unused-variable warning
        }
    }

    tx.commit()?;
    Ok(scan_id)
}

/// Deletes a scan and all its nodes (CASCADE handles the nodes table).
pub fn delete_scan(conn: &Connection, scan_id: i64) -> Result<()> {
    let rows = conn.execute("DELETE FROM scans WHERE id = ?1", params![scan_id])?;
    if rows == 0 {
        return Err(DbError::ScanNotFound(scan_id));
    }
    Ok(())
}

// ─── Read operations ──────────────────────────────────────────────────────────

/// Returns a flat list of every [`DbNode`] belonging to `scan_id`.
///
/// Rows are ordered by `id` (insertion order), which preserves the original
/// depth-first traversal order.
pub fn load_scan_tree(conn: &Connection, scan_id: i64) -> Result<Vec<DbNode>> {
    let mut stmt = conn.prepare(
        "SELECT id, scan_id, parent_id, path, is_dir, size, modified
         FROM   nodes
         WHERE  scan_id = ?1
         ORDER  BY id ASC",
    )?;

    let nodes = stmt
        .query_map(params![scan_id], |row| {
            Ok(DbNode {
                id: Some(row.get(0)?),
                scan_id: row.get(1)?,
                parent_id: row.get(2)?,
                path: row.get(3)?,
                is_dir: row.get::<_, i64>(4)? != 0,
                size: row.get(5)?,
                modified: row.get(6)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(nodes)
}

/// Returns all scans for a given `drive_root`, ordered newest-first.
pub fn get_scans_for_drive(conn: &Connection, drive_root: &str) -> Result<Vec<ScanMeta>> {
    let mut stmt = conn.prepare(
        "SELECT id, drive_root, scanned_at, node_count
         FROM   scans
         WHERE  drive_root = ?1
         ORDER  BY scanned_at DESC",
    )?;

    let scans = stmt
        .query_map(params![drive_root], |row| {
            Ok(ScanMeta {
                id: row.get(0)?,
                drive_root: row.get(1)?,
                scanned_at: row.get(2)?,
                node_count: row.get(3)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(scans)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_nodes(scan_id: i64) -> Vec<DbNode> {
        vec![
            DbNode {
                id: None,
                scan_id,
                parent_id: None,
                path: "/home/user".to_string(),
                is_dir: true,
                size: 0,
                modified: 1_700_000_000,
            },
            DbNode {
                id: None,
                scan_id,
                parent_id: Some(0), // index 0 = the root above
                path: "/home/user/file.txt".to_string(),
                is_dir: false,
                size: 1024,
                modified: 1_700_000_001,
            },
        ]
    }

    #[test]
    fn test_open_in_memory() {
        let conn = open_in_memory().expect("open_in_memory failed");
        // Verify the schema was applied by querying the tables.
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM scans", [], |r| r.get(0))
            .expect("query failed");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_save_and_load_scan() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes(0 /* placeholder, overwritten by save_scan */);
        let scan_id = save_scan(&conn, "C:\\", 1_700_000_000, &nodes).unwrap();
        assert!(scan_id > 0);

        let loaded = load_scan_tree(&conn, scan_id).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].path, "/home/user");
        assert_eq!(loaded[1].path, "/home/user/file.txt");
        assert!(loaded[0].is_dir);
        assert!(!loaded[1].is_dir);
        assert_eq!(loaded[1].size, 1024);
    }

    #[test]
    fn test_get_scans_for_drive() {
        let conn = open_in_memory().unwrap();
        save_scan(&conn, "D:\\", 1_000, &[]).unwrap();
        save_scan(&conn, "D:\\", 2_000, &[]).unwrap();
        save_scan(&conn, "C:\\", 3_000, &[]).unwrap();

        let scans = get_scans_for_drive(&conn, "D:\\").unwrap();
        assert_eq!(scans.len(), 2);
        // Newest first.
        assert_eq!(scans[0].scanned_at, 2_000);
        assert_eq!(scans[1].scanned_at, 1_000);
    }

    #[test]
    fn test_delete_scan_cascade() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes(0);
        let scan_id = save_scan(&conn, "E:\\", 9_999, &nodes).unwrap();

        // Nodes exist before deletion.
        let before = load_scan_tree(&conn, scan_id).unwrap();
        assert_eq!(before.len(), 2);

        delete_scan(&conn, scan_id).unwrap();

        // Nodes must be gone (CASCADE).
        let after = load_scan_tree(&conn, scan_id).unwrap();
        assert!(after.is_empty());
    }

    #[test]
    fn test_delete_nonexistent_scan_returns_error() {
        let conn = open_in_memory().unwrap();
        let result = delete_scan(&conn, 9999);
        assert!(matches!(result, Err(DbError::ScanNotFound(9999))));
    }

    #[test]
    fn test_parent_child_relationship() {
        let conn = open_in_memory().unwrap();
        let nodes = sample_nodes(0);
        let scan_id = save_scan(&conn, "/", 0, &nodes).unwrap();
        let loaded = load_scan_tree(&conn, scan_id).unwrap();

        let root = &loaded[0];
        let child = &loaded[1];

        assert!(root.parent_id.is_none());
        assert_eq!(child.parent_id, root.id);
    }
}