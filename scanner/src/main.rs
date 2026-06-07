//! Scanner binary entry point.
//!
//! Demonstrates the scanner component by scanning the current directory
//! using all three available scanning strategies.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use scanner::{read_dir_immediate, scan_dir_incremental, scan_dir_sync, ScanMessage};

fn main() {
    let root = PathBuf::from(".");

    println!("=== Scanner Demo ===\n");

    // -------------------------------------------------------------------------
    // 1. read_dir_immediate — single directory level
    // -------------------------------------------------------------------------
    println!("--- read_dir_immediate ---");
    match read_dir_immediate(&root) {
        Ok(entries) => {
            println!("Found {} entries in '{}':", entries.len(), root.display());
            for entry in &entries {
                println!("  {:?}", entry);
            }
        }
        Err(e) => eprintln!("read_dir_immediate error: {}", e),
    }
    println!();

    // -------------------------------------------------------------------------
    // 2. scan_dir_sync — recursive single-threaded with bottom-up size aggregation
    // -------------------------------------------------------------------------
    println!("--- scan_dir_sync ---");
    let cancelled = Arc::new(AtomicBool::new(false));
    match scan_dir_sync(&root, Arc::clone(&cancelled)) {
        Ok(node) => {
            println!(
                "Scanned '{}': total size = {} bytes, children = {}",
                node.path.display(),
                node.size,
                node.children.len()
            );
        }
        Err(e) => eprintln!("scan_dir_sync error: {}", e),
    }
    println!();

    // -------------------------------------------------------------------------
    // 3. scan_dir_incremental — multi-threaded with message channel
    // -------------------------------------------------------------------------
    println!("--- scan_dir_incremental ---");
    let cancelled = Arc::new(AtomicBool::new(false));
    let (tx, rx) = std::sync::mpsc::channel::<ScanMessage>();

    let scan_root = root.clone();
    let scan_cancelled = Arc::clone(&cancelled);
    let handle = std::thread::spawn(move || {
        scan_dir_incremental(scan_root, tx, scan_cancelled);
    });

    let mut dir_count: usize = 0;
    let mut error_count: usize = 0;

    for msg in rx {
        match msg {
            ScanMessage::DirScanned(node) => {
                dir_count += 1;
                println!(
                    "  [DirScanned] {} ({} bytes)",
                    node.path.display(),
                    node.size
                );
            }
            ScanMessage::ScanError { path, error } => {
                error_count += 1;
                eprintln!("  [ScanError] {}: {}", path.display(), error);
            }
            ScanMessage::Complete => {
                println!("  [Complete]");
                break;
            }
        }
    }

    handle.join().expect("scanner thread panicked");

    println!(
        "\nIncremental scan finished: {} dirs scanned, {} errors.",
        dir_count, error_count
    );
}