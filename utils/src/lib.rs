//! # utils
//!
//! Utility functions for the Desktop component.
//!
//! ## Features
//!
//! - `format_size`: Converts bytes to human-readable strings (B/KB/MB/GB/TB, 2 decimal places).
//! - `format_number`: Adds comma thousand-separators to integers.
//! - `chrono_now`: Generates an ISO 8601 UTC timestamp without the `chrono` crate.
//! - `format_system_time`: Converts `SystemTime` to ISO 8601 string.
//! - `days_to_ymd`: Converts a day count to (year, month, day) using the Hinnant algorithm.
//! - `enumerate_drives`: Enumerates system drives using `sysinfo` with Windows drive letter normalization.

pub mod formatting;
pub mod time;
pub mod drives;

pub use formatting::{format_size, format_number};
pub use time::{chrono_now, format_system_time, days_to_ymd};
pub use drives::enumerate_drives;