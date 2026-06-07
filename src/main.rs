//! Binary entry point – exercises the database component with a small smoke test.
//!
//! In production this crate is consumed as a library; the binary is provided
//! for quick manual verification and CI smoke-testing.

use database::{open_in_memory, save_scan, load_scan_tree, get_scans_for_drive, delete_scan, ScanNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("database component – smoke test");

    let mut conn = open_in_memory()?;

    // Build a tiny synthetic tree.
    let tree = ScanNode {
        path: "C:\\".into(),
        is_dir: true,
        size_bytes: 0,
        children: vec![
            ScanNode {
                path: "C:\\Users".into(),
                is_dir: true,
                size_bytes: 0,
                children: vec![
                    ScanNode {
                        path: "C:\\Users\\Alice".into(),
                        is_dir: true,
                        size_bytes: 0,
                        children: vec![
                            ScanNode {
                                path: "C:\\Users\\Alice\\document.docx".into(),
                                is_dir: false,
                                size_bytes: 51_200,
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
            ScanNode {
                path: "C:\\Windows".into(),
                is_dir: true,
                size_bytes: 0,
                children: vec![],
            },
        ],
    };

    // Persist the scan.
    let scan_id = save_scan(&mut conn, "C:\\", &tree)?;
    println!("Saved scan with id={scan_id}");

    // Load and display nodes.
    let nodes = load_scan_tree(&conn, scan_id)?;
    println!("Loaded {} node(s):", nodes.len());
    for node in &nodes {
        let kind = if node.is_dir { "DIR " } else { "FILE" };
        println!(
            "  [{kind}] id={:<4} parent={:<6} size={:<10} {}",
            node.id,
            node.parent_id.map_or("none".into(), |p| p.to_string()),
            node.size_bytes,
            node.path,
        );
    }

    // Query scans for the drive.
    let scans = get_scans_for_drive(&conn, "C:\\")?;
    println!("\nScans for C:\\:");
    for s in &scans {
        println!("  id={} at={} nodes={}", s.id, s.scanned_at, s.node_count);
    }

    // Delete the scan.
    delete_scan(&mut conn, scan_id)?;
    println!("\nDeleted scan id={scan_id}");

    let remaining = load_scan_tree(&conn, scan_id)?;
    println!("Nodes remaining after delete: {}", remaining.len());

    println!("\nSmoke test passed ✓");
    Ok(())
}