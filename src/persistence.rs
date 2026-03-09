use anyhow::Result;
use rusqlite::{Connection, params};

use crate::models::{DbNode, FsNode, ScanMeta};

// ---------------------------------------------------------------------------
// Schema
// ---------------------------------------------------------------------------

const SCHEMA: &str = "
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS scans (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    drive       TEXT    NOT NULL,
    scanned_at  TEXT    NOT NULL,
    total_size  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS nodes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id     INTEGER NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
    parent_id   INTEGER,
    name        TEXT    NOT NULL,
    path        TEXT    NOT NULL,
    is_dir      BOOLEAN NOT NULL,
    size        INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_nodes_scan ON nodes(scan_id);
CREATE INDEX IF NOT EXISTS idx_nodes_path ON nodes(path);
";

// ---------------------------------------------------------------------------
// Open / migrate
// ---------------------------------------------------------------------------

pub fn open_db() -> Result<Connection> {
    let db_dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("cannot locate APPDATA"))?
        .join("storage-wars");

    std::fs::create_dir_all(&db_dir)?;
    let conn = Connection::open(db_dir.join("storage-wars.db"))?;
    migrate(&conn)?;
    Ok(conn)
}

pub fn open_in_memory() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    migrate(&conn)?;
    Ok(conn)
}

fn migrate(conn: &Connection) -> Result<()> {
    conn.execute_batch(SCHEMA)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Save a scan
// ---------------------------------------------------------------------------

/// Flatten `root` depth-first and bulk-insert everything in one transaction.
/// Returns the new `scan_id`.
pub fn save_scan(conn: &Connection, drive: &str, root: &FsNode) -> Result<i64> {
    let scanned_at = chrono_now();

    conn.execute(
        "INSERT INTO scans (drive, scanned_at, total_size) VALUES (?1, ?2, ?3)",
        params![drive, scanned_at, root.current_size as i64],
    )?;
    let scan_id = conn.last_insert_rowid();

    let mut stmt = conn.prepare(
        "INSERT INTO nodes (scan_id, parent_id, name, path, is_dir, size)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;

    let mut stack: Vec<(&FsNode, Option<i64>)> = vec![(root, None)];
    while let Some((node, parent_id)) = stack.pop() {
        stmt.execute(params![
            scan_id,
            parent_id,
            node.name,
            node.path.to_string_lossy().as_ref(),
            node.is_dir,
            node.current_size as i64,
        ])?;
        let node_id = conn.last_insert_rowid();
        for child in &node.children {
            stack.push((child, Some(node_id)));
        }
    }

    Ok(scan_id)
}

// ---------------------------------------------------------------------------
// Query helpers
// ---------------------------------------------------------------------------

pub fn get_scans_for_drive(conn: &Connection, drive: &str) -> Result<Vec<ScanMeta>> {
    let mut stmt = conn.prepare(
        "SELECT id, drive, scanned_at, total_size FROM scans WHERE drive = ?1 ORDER BY id DESC",
    )?;
    let rows = stmt.query_map(params![drive], |row| {
        Ok(ScanMeta {
            id: row.get(0)?,
            drive: row.get(1)?,
            scanned_at: row.get(2)?,
            total_size: row.get::<_, i64>(3)? as u64,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

pub fn load_scan_tree(conn: &Connection, scan_id: i64) -> Result<Vec<DbNode>> {
    let mut stmt = conn.prepare(
        "SELECT id, scan_id, parent_id, name, path, is_dir, size
         FROM nodes WHERE scan_id = ?1",
    )?;
    let rows = stmt.query_map(params![scan_id], |row| {
        Ok(DbNode {
            id: row.get(0)?,
            scan_id: row.get(1)?,
            parent_id: row.get(2)?,
            name: row.get(3)?,
            path: row.get(4)?,
            is_dir: row.get(5)?,
            size: row.get::<_, i64>(6)? as u64,
        })
    })?;
    Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
}

pub fn delete_scan(conn: &Connection, scan_id: i64) -> Result<()> {
    conn.execute("DELETE FROM scans WHERE id = ?1", params![scan_id])?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn chrono_now() -> String {
    // std-only ISO 8601 timestamp without pulling in `chrono`.
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Format as YYYY-MM-DDTHH:MM:SSZ (UTC, second precision).
    let s = secs;
    let sec = s % 60;
    let min = (s / 60) % 60;
    let hour = (s / 3600) % 24;
    let days = s / 86400; // days since 1970-01-01
    let (year, month, day) = days_to_ymd(days);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}Z")
}

/// Converts days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FsNode;
    use std::path::PathBuf;

    fn simple_tree() -> FsNode {
        FsNode {
            name: "root".into(),
            path: PathBuf::from("C:/"),
            is_dir: true,
            current_size: 300,
            prev_size: None,
            children: vec![
                FsNode {
                    name: "docs".into(),
                    path: PathBuf::from("C:/docs"),
                    is_dir: true,
                    current_size: 200,
                    prev_size: None,
                    children: vec![FsNode {
                        name: "readme.txt".into(),
                        path: PathBuf::from("C:/docs/readme.txt"),
                        is_dir: false,
                        current_size: 100,
                        prev_size: None,
                        children: vec![],
                    }],
                },
                FsNode {
                    name: "empty".into(),
                    path: PathBuf::from("C:/empty"),
                    is_dir: true,
                    current_size: 0,
                    prev_size: None,
                    children: vec![],
                },
            ],
        }
    }

    #[test]
    fn schema_migration_runs() {
        open_in_memory().expect("schema migration should succeed");
    }

    #[test]
    fn save_and_load_roundtrip() {
        let conn = open_in_memory().unwrap();
        let tree = simple_tree();
        let scan_id = save_scan(&conn, "C:", &tree).unwrap();

        let nodes = load_scan_tree(&conn, scan_id).unwrap();
        // root + docs + readme.txt + empty = 4 nodes
        assert_eq!(nodes.len(), 4);

        let root_node = nodes.iter().find(|n| n.name == "root").expect("root missing");
        assert_eq!(root_node.size, 300);
        assert!(root_node.parent_id.is_none());

        let file_node = nodes.iter().find(|n| n.name == "readme.txt").expect("file missing");
        assert_eq!(file_node.size, 100);
        assert!(!file_node.is_dir);
    }

    #[test]
    fn get_scans_for_drive_filters_correctly() {
        let conn = open_in_memory().unwrap();
        let tree = simple_tree();

        save_scan(&conn, "C:", &tree).unwrap();
        save_scan(&conn, "C:", &tree).unwrap();
        save_scan(&conn, "D:", &tree).unwrap();

        let c_scans = get_scans_for_drive(&conn, "C:").unwrap();
        assert_eq!(c_scans.len(), 2);
        assert!(c_scans.iter().all(|s| s.drive == "C:"));

        let d_scans = get_scans_for_drive(&conn, "D:").unwrap();
        assert_eq!(d_scans.len(), 1);

        let e_scans = get_scans_for_drive(&conn, "E:").unwrap();
        assert!(e_scans.is_empty());
    }

    #[test]
    fn delete_scan_cascades_to_nodes() {
        let conn = open_in_memory().unwrap();
        // Foreign key enforcement must be on for CASCADE to fire.
        conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

        let tree = simple_tree();
        let scan_id = save_scan(&conn, "C:", &tree).unwrap();

        let before = load_scan_tree(&conn, scan_id).unwrap();
        assert!(!before.is_empty());

        delete_scan(&conn, scan_id).unwrap();

        let after = load_scan_tree(&conn, scan_id).unwrap();
        assert!(after.is_empty(), "nodes should be cascade-deleted");

        let scans = get_scans_for_drive(&conn, "C:").unwrap();
        assert!(scans.is_empty(), "scan row should be deleted");
    }

    #[test]
    fn scan_meta_total_size_stored() {
        let conn = open_in_memory().unwrap();
        let tree = simple_tree();
        save_scan(&conn, "C:", &tree).unwrap();

        let scans = get_scans_for_drive(&conn, "C:").unwrap();
        assert_eq!(scans[0].total_size, 300);
    }
}
