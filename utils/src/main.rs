//! Entry point for the `utils` binary.
//!
//! Demonstrates all utility functions and prints results to stdout.

use utils::{
    chrono_now, enumerate_drives, format_number, format_size, format_system_time,
};
use std::time::SystemTime;

fn main() {
    println!("=== utils — Desktop Utility Functions ===\n");

    // --- format_size ---
    println!("[ format_size ]");
    let sizes: &[u64] = &[
        0,
        512,
        1_024,
        1_048_576,
        1_073_741_824,
        1_099_511_627_776,
        1_500_000_000_000,
    ];
    for &bytes in sizes {
        println!("  {:>20} bytes  →  {}", format_number(bytes), format_size(bytes));
    }
    println!();

    // --- format_number ---
    println!("[ format_number ]");
    let numbers: &[u64] = &[0, 42, 999, 1_000, 12_345, 1_234_567, 9_876_543_210];
    for &n in numbers {
        println!("  {:>15}  →  {}", n, format_number(n));
    }
    println!();

    // --- chrono_now ---
    println!("[ chrono_now ]");
    println!("  Current UTC timestamp: {}", chrono_now());
    println!();

    // --- format_system_time ---
    println!("[ format_system_time ]");
    let now = SystemTime::now();
    println!("  SystemTime::now()  →  {}", format_system_time(now));
    println!();

    // --- enumerate_drives ---
    println!("[ enumerate_drives ]");
    let drives = enumerate_drives();
    if drives.is_empty() {
        println!("  (no drives detected)");
    } else {
        for drive in &drives {
            let used = drive.total_bytes.saturating_sub(drive.available_bytes);
            println!(
                "  {}  total: {}  used: {}  free: {}",
                drive.name,
                format_size(drive.total_bytes),
                format_size(used),
                format_size(drive.available_bytes),
            );
        }
    }
    println!();

    println!("=== done ===");
}