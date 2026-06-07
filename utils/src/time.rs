//! Time utilities: ISO 8601 UTC timestamps without the `chrono` crate.

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current UTC time as an ISO 8601 string (`YYYY-MM-DDTHH:MM:SSZ`).
///
/// Does **not** depend on the `chrono` crate; uses only `std::time`.
///
/// # Examples
///
///