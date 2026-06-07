use std::time::{SystemTime, UNIX_EPOCH};

/// Formats a byte count into a human-readable string with 2 decimal places.
///
/// Units: B, KB, MB, GB, TB.
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[0])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Formats an integer with comma thousand-separators.
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}

/// Converts a count of days since the Unix epoch (1970-01-01) into (year, month, day)
/// using Howard Hinnant's algorithm.
fn days_to_ymd(epoch_days: i64) -> (i64, u32, u32) {
    let z = epoch_days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = (z - era * 146_097) as u32; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // year of era [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// Returns the current UTC timestamp as a `String` in `YYYY-MM-DD HH:MM:SS` format.
///
/// Uses `std::time::SystemTime` and the Hinnant algorithm — no `chrono` dependency.
pub fn chrono_now() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let total_secs = duration.as_secs();

    let epoch_days = (total_secs / 86_400) as i64;
    let day_secs = total_secs % 86_400;

    let hours = day_secs / 3600;
    let minutes = (day_secs % 3600) / 60;
    let seconds = day_secs % 60;

    let (year, month, day) = days_to_ymd(epoch_days);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

/// Converts a `SystemTime` to an ISO 8601 UTC string (`YYYY-MM-DD HH:MM:SS`).
pub fn format_system_time(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let total_secs = duration.as_secs();

    let epoch_days = (total_secs / 86_400) as i64;
    let day_secs = total_secs % 86_400;

    let hours = day_secs / 3600;
    let minutes = (day_secs % 3600) / 60;
    let seconds = day_secs % 60;

    let (year, month, day) = days_to_ymd(epoch_days);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1_048_576), "1.00 MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn test_format_size_tb() {
        assert_eq!(format_size(1_099_511_627_776), "1.00 TB");
    }

    #[test]
    fn test_format_number_small() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(999), "999");
    }

    #[test]
    fn test_format_number_thousands() {
        assert_eq!(format_number(1_000), "1,000");
        assert_eq!(format_number(12_345), "12,345");
        assert_eq!(format_number(1_234_567), "1,234,567");
        assert_eq!(format_number(1_000_000_000), "1,000,000,000");
    }

    #[test]
    fn test_chrono_now_format() {
        let ts = chrono_now();
        // Must match "YYYY-MM-DD HH:MM:SS"
        assert_eq!(ts.len(), 19);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], " ");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
    }

    #[test]
    fn test_format_system_time_epoch() {
        let epoch = UNIX_EPOCH;
        assert_eq!(format_system_time(epoch), "1970-01-01 00:00:00");
    }
}