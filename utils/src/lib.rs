//! # Utils
//!
//! Utility functions for the Desktop component.
//!
//! ## Features
//!
//! - `format_size`: Converts bytes to human-readable string (B/KB/MB/GB/TB, 2 decimal places)
//! - `format_number`: Adds comma thousand-separators to integers
//! - `chrono_now`: Generates ISO 8601 UTC timestamp without the `chrono` crate
//! - `format_system_time`: Converts `SystemTime` to ISO 8601 string
//! - `days_to_ymd`: Converts days since epoch to (year, month, day) using the Hinnant algorithm
//! - `enumerate_drives`: Enumerates system drives using `sysinfo` with Windows drive letter normalization

use std::time::{SystemTime, UNIX_EPOCH};

use sysinfo::Disks;

// ---------------------------------------------------------------------------
// format_size
// ---------------------------------------------------------------------------

/// Converts a byte count into a human-readable string with 2 decimal places.
///
/// Units: B, KB, MB, GB, TB.
///
/// # Examples
///
///