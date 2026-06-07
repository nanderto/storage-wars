use chrono::Local;

/// Formats a byte count into a human-readable string with 2 decimal places.
///
/// Uses binary-style labels (B, KB, MB, GB, TB) with 1024-based divisions.
///
/// # Examples
///
///