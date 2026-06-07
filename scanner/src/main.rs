use scanner::{scan_dir_incremental, ScanMessage};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let root = if args.len() > 1 {
        std::path::PathBuf::from(&args[1])
    } else {
        std::env::current_dir().expect("Failed to get current directory")
    };

    println!("Scanning: {}", root.display());

    let cancelled = Arc::new(AtomicBool::new(false));
    let (tx, rx) = std::sync::mpsc::channel::<ScanMessage>();

    let cancelled_clone = Arc::clone(&cancelled);
    let scan_thread = std::thread::spawn(move || {
        scan_dir_incremental(root, tx, cancelled_clone);
    });

    let mut dir_count = 0usize;
    let mut error_count = 0usize;

    for msg in rx {
        match msg {
            ScanMessage::DirScanned(entry) => {
                dir_count += 1;
                println!(
                    "[DIR] {} ({} bytes)",
                    entry.path.display(),
                    entry.size
                );
            }
            ScanMessage::ScanError { path, error } => {
                error_count += 1;
                eprintln!("[ERR] {}: {}", path.display(), error);
            }
            ScanMessage::Complete => {
                println!(
                    "\nScan complete. Directories: {}, Errors: {}",
                    dir_count, error_count
                );
                break;
            }
        }
    }

    scan_thread.join().expect("Scan thread panicked");
}