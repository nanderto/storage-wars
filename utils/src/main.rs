//! Entry point for the utils binary.
//!
//! Demonstrates all utility functions provided by this crate.

use utils::{
    format_size,
    format_number,
    chrono_now,
    format_system_time,
    days_to_ymd,
    enumerate_drives,
};
use std::time::SystemTime;

fn main() {
    // --- format_size ---
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
        println!("  {} bytes => {}", bytes, format_size(bytes));
    }

    // --- format_number ---
    println!("\n=== format_number ===");
    let numbers: &[u64] = &[0, 999, 1_000, 1_234_567, 9_876_543_210];
    for &n in numbers {
        println!("  {} => {}", n, format_number(n));
    }

    // --- chrono_now ---
    println!("\n=== chrono_now ===");
    let now = chrono_now();
    println!("  Current UTC timestamp: {}", now);

    // --- format_system_time ---
    println!("\n=== format_system_time ===");
    let sys_time = SystemTime::now();
    let formatted = format_system_time(sys_time);
    println!("  SystemTime formatted: {}", formatted);

    // --- days_to_ymd ---
    println!("\n=== days_to_ymd ===");
    // Day 0 in the Hinnant algorithm corresponds to 1970-01-01
    let test_days: &[i64] = &[0, 1, 365, 366, 18628];
    for &d in test_days {
        let (y, m, day) = days_to_ymd(d);
        println!("  days={} => {:04}-{:02}-{:02}", d, y, m, day);
    }

    // --- enumerate_drives ---
    println!("\n=== enumerate_drives ===");
    let drives = enumerate_drives();
    if drives.is_empty() {
        println!("  No drives found.");
    } else {
        for drive in &drives {
            println!("  Drive: {}", drive);
        }
    }
}