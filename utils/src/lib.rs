//! Utility functions for the Desktop component.
//!
//! Provides:
//! - `format_size`: converts bytes to human-readable string (B/KB/MB/GB/TB, 2 decimal places)
//! - `format_number`: adds comma thousand-separators to integers
//! - `chrono_now`: generates ISO 8601 UTC timestamp without the chrono crate
//! - `format_system_time`: converts `std::time::SystemTime` to ISO 8601
//! - `days_to_ymd`: converts a day count to (year, month, day) using the Hinnant algorithm
//! - `enumerate_drives`: lists available drives using sysinfo with Windows drive letter normalization

pub mod formatting;
pub mod time_utils;
pub mod drives;

pub use formatting::{format_size, format_number};
pub use time_utils::{chrono_now, format_system_time, days_to_ymd};
pub use drives::enumerate_drives;