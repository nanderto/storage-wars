//! Time and timestamp utilities — ISO 8601 UTC, without the `chrono` crate.

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current UTC time as an ISO 8601 string (`YYYY-MM-DDTHH:MM:SSZ`).
///
/// Does **not** use the `chrono` crate; relies solely on [`std::time::SystemTime`].
///
/// # Examples
///
///