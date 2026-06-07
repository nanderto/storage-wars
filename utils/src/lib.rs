//! Utility functions for the Desktop component.
//!
//! Provides:
//! - `format_size`: converts bytes to human-readable strings (B/KB/MB/GB/TB, 2 decimal places)
//! - `format_number`: adds comma thousand-separators to integers
//! - `chrono_now`: generates ISO 8601 UTC timestamp without the chrono crate
//! - `format_system_time`: converts `SystemTime` to ISO 8601
//! - `days_to_ymd`: converts a day count to (year, month, day) using the Hinnant algorithm
//! - `enumerate_drives`: lists drives using sysinfo with Windows drive letter normalization

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