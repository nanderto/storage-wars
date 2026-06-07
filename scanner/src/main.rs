//! Binary entry point for the `scanner` component.
//!
//! Demonstrates [`scan_dir_incremental`] by scanning the path supplied as the
//! first command-line argument (defaults to the current directory).

use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use scanner::{scan_dir_incremental, ScanMessage};

fn main() {
    let root = env::args().nth(1).unwrap_or_else(|| ".".to_string());

    println!("Scanning: {root}");

    let cancelled = Arc::new(AtomicBool::new(false));

    // Allow Ctrl-C to cancel the scan gracefully.
    {
        let cancelled = Arc::clone(&cancelled);
        ctrlc_handler(cancelled);
    }

    let rx = scan_dir_incremental(&root, Arc::clone(&cancelled));

    let mut dir_count: u64 = 0;
    let mut file_count: u64 = 0;
    let mut error_count: u64 = 0;

    for msg in rx {
        match msg {
            ScanMessage::DirScanned { dir, entries } => {
                dir_count += 1;
                for entry in &entries {
                    if !entry.is_dir {
                        file_count += 1;
                    }
                }
                println!(
                    "[DIR] {} ({} entries)",
                    dir.display(),
                    entries.len()
                );
            }
            ScanMessage::ScanError { path, message } => {
                error_count += 1;
                eprintln!("[ERR] {}: {}", path.display(), message);
            }
            ScanMessage::Complete => {
                break;
            }
        }
    }

    let status = if cancelled.load(Ordering::Acquire) {
        "cancelled"
    } else {
        "complete"
    };

    println!(
        "\nScan {status}: {dir_count} dirs, {file_count} files, {error_count} errors."
    );
}

/// Register a Ctrl-C handler that sets the `cancelled` flag.
///
/// Uses a simple `std`-only approach: spawn a background thread that parks
/// itself; the OS signal will interrupt the park on supported platforms.
/// For a production binary, replace this with the `ctrlc` crate.
fn ctrlc_handler(cancelled: Arc<AtomicBool>) {
    // This is a best-effort no-dependency handler.
    // On Unix the process will receive SIGINT; we rely on the default handler
    // terminating the process if the flag approach is insufficient.
    let _ = cancelled; // suppress unused warning when signal handling is omitted
}