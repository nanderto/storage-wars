//! # utils
//!
//! Utility functions for the Desktop component.
//!
//! ## Features
//!
//! - `format_size`: Converts bytes to human-readable string (B/KB/MB/GB/TB, 2 decimal places)
//! - `format_number`: Adds comma thousand-separators to integers
//! - `chrono_now`: Generates ISO 8601 UTC timestamp without the `chrono` crate
//! - `format_system_time`: Converts `SystemTime` to ISO 8601 string
//! - `days_to_ymd`: Converts days since epoch to (year, month, day) using Hinnant algorithm
//! - `enumerate_drives`: Enumerates system drives using `sysinfo` with Windows drive letter normalization

pub mod format;
pub mod time;
pub mod drives;

pub use format::{format_size, format_number};
pub use time::{chrono_now, format_system_time, days_to_ymd};
pub use drives::enumerate_drives;