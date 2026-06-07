# utils

Desktop utility library written in Rust, using [`sysinfo`](https://crates.io/crates/sysinfo).

## Features

| Function | Description |
|---|---|
| `format_size(bytes: u64) -> String` | Converts bytes to human-readable string (B / KB / MB / GB / TB, 2 d.p.) |
| `format_number(n: u64) -> String` | Adds comma thousand-separators |
| `chrono_now() -> String` | ISO 8601 UTC timestamp — no `chrono` dependency |
| `format_system_time(t: SystemTime) -> String` | Converts `SystemTime` to ISO 8601 |
| `days_to_ymd(days: u64) -> (u32, u32, u32)` | Hinnant civil-from-days algorithm |
| `enumerate_drives() -> Vec<String>` | Lists system disks via `sysinfo`; normalises Windows drive letters |

## Build

Run `cargo build` from the `utils/` directory.

Run `cargo test` to execute the full test suite.

Run `cargo run` to execute the demonstration binary.

## Project Layout