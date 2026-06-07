//! Binary entry point for the `utils` crate.
//!
//! Demonstrates all public utilities:
//! - [`format_size`]
//! - [`format_number`]
//! - [`chrono_now`]
//! - [`format_system_time`]
//! - [`enumerate_drives`]

use utils::{
    chrono_now, enumerate_drives, format_number, format_size, format_system_time,
};
use std::time::UNIX_EPOCH;

fn main() {
    // ── format_size ──────────────────────────────────────────────────────────
    println!("=== format_size ===");
    let sizes: &[u64] = &[
        0,
        512,
        1_024,
        1_536,
        1_048_576,
        1_073_741_824,
        1_099_511_627_776,
    ];
    for &bytes in sizes {
        println!("  {:>20} bytes  →  {}", bytes, format_size(bytes));
    }

    // ── format_number ────────────────────────────────────────────────────────
    println!("\n=== format_number ===");
    let numbers: &[u64] = &[0, 999, 1_000, 1_234_567, 1_234_567_890, u64::MAX];
    for &n in numbers {
        println!("  {:>25}  →  {}", n, format_number(n));
    }

    // ── chrono_now ───────────────────────────────────────────────────────────
    println!("\n=== chrono_now ===");
    println!("  Current UTC: {}", chrono_now());

    // ── format_system_time ───────────────────────────────────────────────────
    println!("\n=== format_system_time ===");
    println!("  Unix epoch : {}", format_system_time(UNIX_EPOCH));
    println!(
        "  Now        : {}",
        format_system_time(std::time::SystemTime::now())
    );

    // ── enumerate_drives ─────────────────────────────────────────────────────
    println!("\n=== enumerate_drives ===");
    let drives = enumerate_drives();
    if drives.is_empty() {
        println!("  (no drives detected)");
    } else {
        for drive in &drives {
            println!(
                "  {}  total={}  available={}",
                drive.name,
                format_size(drive.total_bytes),
                format_size(drive.available_bytes),
            );
        }
    }
}