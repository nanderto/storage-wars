//! Time utilities: ISO 8601 UTC timestamps and calendar conversion.
//!
//! Deliberately avoids the `chrono` crate; uses only `std::time`.

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current UTC time as an ISO 8601 string (`YYYY-MM-DDTHH:MM:SSZ`).
///
/// Does **not** use the `chrono` crate.
///
/// # Examples
///
///