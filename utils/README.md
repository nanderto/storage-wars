# utils

Utility functions for the Desktop component, written in Rust.

## Features

| Function | Description |
|---|---|
| `format_size(bytes: u64) -> String` | Converts bytes to human-readable string (B/KB/MB/GB/TB, 2 decimal places) |
| `format_number(n: u64) -> String` | Adds comma thousand-separators to an integer |
| `chrono_now() -> String` | Returns current UTC time as ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) without `chrono` |
| `format_system_time(t: SystemTime) -> String` | Converts `SystemTime` to ISO 8601 UTC string |
| `days_to_ymd(days: i64) -> (i64, u32, u32)` | Hinnant algorithm: days since epoch → (year, month, day) |
| `enumerate_drives() -> Vec<String>` | Lists drives via `sysinfo`; normalizes Windows drive letters |

## Dependencies

- [`sysinfo`](https://crates.io/crates/sysinfo) `0.30` — cross-platform system information

## Build