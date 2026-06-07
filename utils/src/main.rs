//! Binary entry point for the `utils` crate.
//!
//! Demonstrates each utility function and prints the results to stdout.

use utils::{
    chrono_now, enumerate_drives, format_number, format_size, format_system_time,
};
use std::time::SystemTime;

fn main() {
    println!("=== utils demo ===\n");

    // format_size
    println!("--- format_size ---");
    for &bytes in &[0u64, 512, 1_024, 1_048_576, 1_073_741_824, 1_099_511_627_776] {
        println!("  {:>15} bytes  =>  {}", bytes, format_size(bytes));
    }

    println!();

    // format_number
    println!("--- format_number ---");
    for &n in &[0u64, 999, 1_000, 10_000, 1_234_567, 9_876_543_210] {
        println!("  {:>15}  =>  {}", n, format_number(n));
    }

    println!();

    // chrono_now
    println!("--- chrono_now ---");
    println!("  Current UTC: {}", chrono_now());

    println!();

    // format_system_time
    println!("--- format_system_time ---");
    println!("  SystemTime::now() => {}", format_system_time(SystemTime::now()));

    println!();

    // enumerate_drives
    println!("--- enumerate_drives ---");
    let drives = enumerate_drives();
    if drives.is_empty() {
        println!("  (no drives detected)");
    } else {
        for d in &drives {
            println!(
                "  {}  total={}  free={}",
                d.mount_point,
                format_size(d.total_bytes),
                format_size(d.available_bytes),
            );
        }
    }

    println!("\n=== done ===");
}