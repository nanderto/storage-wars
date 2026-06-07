//! Entry point for the `utils` binary.
//!
//! Demonstrates all utility functions provided by the `utils` library.

use utils::{
    format_size,
    format_number,
    chrono_now,
    format_system_time,
    enumerate_drives,
};
use std::time::SystemTime;

fn main() {
    println!("=== utils — Desktop Utility Functions ===\n");

    // format_size: bytes → human-readable
    println!("--- format_size ---");
    let sizes: &[u64] = &[
        0,
        512,
        1_024,
        1_536,
        1_048_576,
        1_073_741_824,
        1_099_511_627_776,
        2_748_779_069_440,
    ];
    for &bytes in sizes {
        println!("  {:>20} bytes  →  {}", bytes, format_size(bytes));
    }

    // format_number: comma thousand-separators
    println!("\n--- format_number ---");
    let numbers: &[u64] = &[0, 999, 1_000, 12_345, 1_000_000, 9_876_543_210];
    for &n in numbers {
        println!("  {:>15}  →  {}", n, format_number(n));
    }

    // chrono_now: ISO 8601 UTC timestamp
    println!("\n--- chrono_now ---");
    println!("  Current UTC timestamp: {}", chrono_now());

    // format_system_time: SystemTime → ISO 8601
    println!("\n--- format_system_time ---");
    let now = SystemTime::now();
    println!("  SystemTime::now() → {}", format_system_time(now));

    // enumerate_drives: list system drives
    println!("\n--- enumerate_drives ---");
    let drives = enumerate_drives();
    if drives.is_empty() {
        println!("  (no drives detected)");
    } else {
        for drive in &drives {
            println!("  {}", drive);
        }
    }

    println!("\n=== Done ===");
}