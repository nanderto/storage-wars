//! Core database operations.

use rusqlite::{params, Connection};

use crate::error::DbError;
use crate::models::{DbNode, ScanMeta, ScanNode};

// ──────────────────────────────────────────────────────────────────────────────
// Schema
// ──────────────────────────────────────────────────────────────────────────────

const SCHEMA_SQL: &str = r#"
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS scans (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    drive_root  TEXT    NOT NULL,
    scanned_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS nodes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id     INTEGER NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
    parent_id   INTEGER REFERENCES nodes(id) ON DELETE CASCADE,
    path        TEXT    NOT NULL,
    is_dir      INTEGER NOT NULL CHECK(is_dir IN (0, 1)),
    size_bytes  INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_nodes_scan_id ON nodes(scan_id);
CREATE INDEX IF NOT EXISTS idx_nodes_path    ON nodes(path);
"#;

// ──────────────────────────────────────────────────────────────────────────────
// Connection helpers
// ──────────────────────────────────────────────────────────────────────────────

/// Open (or create) the production database stored under the OS application-data
/// directory and run the migration DDL.
///
/// | OS      | Location                                          |
/// |---------|---------------------------------------------------|
/// | Windows | `%APPDATA%\disk-scanner\database.db`              |
/// | macOS   | `~/Library/Application Support/disk-scanner/database.db` |
/// | Linux   | `~/.local/share/disk-scanner/database.db`         |
pub fn open_db() -> Result<Connection, DbError> {
    let base = app_data_dir()?;
    std::fs::create_dir_all(&base)?;
    let path = base.join("database.db");
    let conn = Connection::open(path)?;
    migrate(&conn)?;
    Ok(conn)
}

/// Open an **in-memory** database and run the migration DDL.
/// Intended for unit / integration tests only.
pub fn open_in_memory() -> Result<Connection, DbError> {
    let conn = Connection::open_in_memory()?;
    migrate(&conn)?;
    Ok(conn)
}

/// Apply the schema DDL (idempotent – uses `CREATE … IF NOT EXISTS`).
fn migrate(conn: &Connection) -> Result<(), DbError> {
    conn.execute_batch(SCHEMA_SQL)?;
    Ok(())
}

/// Resolve the platform-specific application-data directory.
fn app_data_dir() -> Result<std::path::PathBuf, DbError> {
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("APPDATA")
        .map(std::path::PathBuf::from)
        .ok_or(DbError::AppDataNotFound)?;

    #[cfg(target_os = "macos")]
    let base = {
        let home = std::env::var_os("HOME")
            .map(std::path::PathBuf::from)
            .ok_or(DbError::AppDataNotFound)?;
        home.join("Library").join("Application Support")
    };

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let base = {
        // XDG_DATA_HOME or ~/.local/share
        let xdg = std::env::var_os("XDG_DATA_HOME").map(std::path::PathBuf::from);
        match xdg {
            Some(p) => p,
            None => {
                let home = std::env::var_os("HOME")
                    .map(std::path::PathBuf::from)
                    .ok_or(DbError::AppDataNotFound)?;
                home.join(".local").join("share")
            }
        }
    };

    Ok(base.join("disk-scanner"))
}

// ──────────────────────────────────────────────────────────────────────────────
// Write operations
// ──────────────────────────────────────────────────────────────────────────────

/// Persist a complete scan tree to the database inside a single transaction.
///
/// Nodes are inserted depth-first so that each parent row already exists when
/// its children are inserted.  Returns the newly created `scan_id`.
pub fn save_scan(
    conn: &mut Connection,
    drive_root: &str,
    root: &ScanNode,
) -> Result<i64, DbError> {
    let tx = conn.transaction()?;

    // Insert the scan header row.
    tx.execute(
        "INSERT INTO scans (drive_root) VALUES (?1)",
        params![drive_root],
    )?;
    let scan_id = tx.last_insert_rowid();

    // Depth-first insertion starting from the root (no parent).
    insert_node_recursive(&tx, scan_id, None, root)?;

    tx.commit()?;
    Ok(scan_id)
}

/// Recursively insert a [`ScanNode`] and all its descendants.
fn insert_node_recursive(
    conn: &Connection,
    scan_id: i64,
    parent_id: Option<i64>,
    node: &ScanNode,
) -> Result<i64, DbError> {
    conn.execute(
        "INSERT INTO nodes (scan_id, parent_id, path, is_dir, size_bytes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            scan_id,
            parent_id,
            node.path,
            node.is_dir as i64,
            node.size_bytes,
        ],
    )?;
    let node_id = conn.last_insert_rowid();

    for child in &node.children {
        insert_node_recursive(conn, scan_id, Some(node_id), child)?;
    }

    Ok(node_id)
}

// ──────────────────────────────────────────────────────────────────────────────
// Read operations
// ──────────────────────────────────────────────────────────────────────────────

/// Load all nodes for a given `scan_id` as a flat list ordered by `id`.
pub fn load_scan_tree(conn: &Connection, scan_id: i64) -> Result<Vec<DbNode>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, scan_id, parent_id, path, is_dir, size_bytes
         FROM   nodes
         WHERE  scan_id = ?1
         ORDER  BY id",
    )?;

    let rows = stmt.query_map(params![scan_id], |row| {
        Ok(DbNode {
            id: row.get(0)?,
            scan_id: row.get(1)?,
            parent_id: row.get(2)?,
            path: row.get(3)?,
            is_dir: row.get::<_, i64>(4)? != 0,
            size_bytes: row.get(5)?,
        })
    })?;

    let mut nodes = Vec::new();
    for row in rows {
        nodes.push(row?);
    }
    Ok(nodes)
}

/// Return metadata for every scan whose `drive_root` matches, ordered by
/// `scanned_at` descending (most recent first).
pub fn get_scans_for_drive(
    conn: &Connection,
    drive_root: &str,
) -> Result<Vec<ScanMeta>, DbError> {
    let mut stmt = conn.prepare(
        "SELECT s.id,
                s.drive_root,
                s.scanned_at,
                COUNT(n.id) AS node_count
         FROM   scans s
         LEFT   JOIN nodes n ON n.scan_id = s.id
         WHERE  s.drive_root = ?1
         GROUP  BY s.id
         ORDER  BY s.scanned_at DESC",
    )?;

    let rows = stmt.query_map(params![drive_root], |row| {
        Ok(ScanMeta {
            id: row.get(0)?,
            drive_root: row.get(1)?,
            scanned_at: row.get(2)?,
            node_count: row.get(3)?,
        })
    })?;

    let mut scans = Vec::new();
    for row in rows {
        scans.push(row?);
    }
    Ok(scans)
}

// ──────────────────────────────────────────────────────────────────────────────
// Delete operations
// ──────────────────────────────────────────────────────────────────────────────

/// Delete a scan and all its nodes (CASCADE handles the `nodes` rows).
///
/// Returns [`DbError::ScanNotFound`] when no row with the given `scan_id`
/// exists.
pub fn delete_scan(conn: &mut Connection, scan_id: i64) -> Result<(), DbError> {
    // Enable foreign keys for this connection so CASCADE fires.
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let affected = conn.execute("DELETE FROM scans WHERE id = ?1", params![scan_id])?;

    if affected == 0 {
        return Err(DbError::ScanNotFound(scan_id));
    }
    Ok(())
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> ScanNode {
        ScanNode {
            path: "/mnt/data".into(),
            is_dir: true,
            size_bytes: 0,
            children: vec![
                ScanNode {
                    path: "/mnt/data/docs".into(),
                    is_dir: true,
                    size_bytes: 0,
                    children: vec![ScanNode {
                        path: "/mnt/data/docs/readme.txt".into(),
                        is_dir: false,
                        size_bytes: 1024,
                        children: vec![],
                    }],
                },
                ScanNode {
                    path: "/mnt/data/photo.jpg".into(),
                    is_dir: false,
                    size_bytes: 204_800,
                    children: vec![],
                },
            ],
        }
    }

    #[test]
    fn test_open_in_memory() {
        open_in_memory().expect("in-memory db should open");
    }

    #[test]
    fn test_save_and_load_scan() {
        let mut conn = open_in_memory().unwrap();
        let tree = sample_tree();
        let scan_id = save_scan(&mut conn, "/mnt/data", &tree).unwrap();

        let nodes = load_scan_tree(&conn, scan_id).unwrap();
        // root + docs dir + readme.txt + photo.jpg = 4 nodes
        assert_eq!(nodes.len(), 4);

        let root = nodes.iter().find(|n| n.parent_id.is_none()).unwrap();
        assert!(root.is_dir);
        assert_eq!(root.path, "/mnt/data");
    }

    #[test]
    fn test_get_scans_for_drive() {
        let mut conn = open_in_memory().unwrap();
        let tree = sample_tree();

        save_scan(&mut conn, "/mnt/data", &tree).unwrap();
        save_scan(&mut conn, "/mnt/data", &tree).unwrap();
        save_scan(&mut conn, "/other", &tree).unwrap();

        let scans = get_scans_for_drive(&conn, "/mnt/data").unwrap();
        assert_eq!(scans.len(), 2);
        // Each scan has 4 nodes.
        assert_eq!(scans[0].node_count, 4);
        assert_eq!(scans[0].drive_root, "/mnt/data");
    }

    #[test]
    fn test_delete_scan_cascade() {
        let mut conn = open_in_memory().unwrap();
        let tree = sample_tree();
        let scan_id = save_scan(&mut conn, "/mnt/data", &tree).unwrap();

        // Nodes exist before deletion.
        assert!(!load_scan_tree(&conn, scan_id).unwrap().is_empty());

        delete_scan(&mut conn, scan_id).unwrap();

        // Nodes should be gone (CASCADE).
        assert!(load_scan_tree(&conn, scan_id).unwrap().is_empty());
    }

    #[test]
    fn test_delete_nonexistent_scan() {
        let mut conn = open_in_memory().unwrap();
        let result = delete_scan(&mut conn, 9999);
        assert!(matches!(result, Err(DbError::ScanNotFound(9999))));
    }

    #[test]
    fn test_node_parent_child_relationship() {
        let mut conn = open_in_memory().unwrap();
        let tree = sample_tree();
        let scan_id = save_scan(&mut conn, "/mnt/data", &tree).unwrap();

        let nodes = load_scan_tree(&conn, scan_id).unwrap();

        let root = nodes.iter().find(|n| n.path == "/mnt/data").unwrap();
        let docs = nodes.iter().find(|n| n.path == "/mnt/data/docs").unwrap();
        let readme = nodes
            .iter()
            .find(|n| n.path == "/mnt/data/docs/readme.txt")
            .unwrap();

        assert_eq!(docs.parent_id, Some(root.id));
        assert_eq!(readme.parent_id, Some(docs.id));
        assert_eq!(readme.size_bytes, 1024);
    }
}