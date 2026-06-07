/// Formats a byte count into a human-readable string with 2 decimal places.
///
/// Uses units: B, KB, MB, GB, TB.
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    for (i, unit) in UNITS.iter().enumerate() {
        if size < 1024.0 || i == UNITS.len() - 1 {
            return format!("{:.2} {}", size, unit);
        }
        size /= 1024.0;
    }
    unreachable!()
}

/// Formats a number with comma thousand-separators.
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

/// Returns the current UTC timestamp as a `String` in `YYYY-MM-DD HH:MM:SS` format.
///
/// Implemented without the `chrono` crate, using `std::time::SystemTime` and the
/// Hinnant civil-from-days algorithm.
pub fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let total_secs = duration.as_secs();

    let secs_per_day: u64 = 86_400;
    let day_count = (total_secs / secs_per_day) as i64;
    let remaining = total_secs % secs_per_day;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    let (year, month, day) = days_to_ymd(day_count);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

/// Converts a `std::time::SystemTime` to an ISO 8601 UTC string (`YYYY-MM-DD HH:MM:SS`).
pub fn format_system_time(time: std::time::SystemTime) -> String {
    use std::time::UNIX_EPOCH;

    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let total_secs = duration.as_secs();

    let secs_per_day: u64 = 86_400;
    let day_count = (total_secs / secs_per_day) as i64;
    let remaining = total_secs % secs_per_day;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    let (year, month, day) = days_to_ymd(day_count);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

/// Converts a Unix epoch day count to `(year, month, day)` using the Hinnant
/// civil-from-days algorithm.
///
/// `epoch_days` is the number of days since 1970-01-01 (may be negative).
pub fn days_to_ymd(epoch_days: i64) -> (i64, u32, u32) {
    // Shift so that day 0 == 0000-03-01 (computational epoch)
    let z = epoch_days + 719_468;
    let era: i64 = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe: u32 = (z - era * 146_097) as u32; // day of era  [0, 146096]
    let yoe: u32 =
        (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // year of era [0, 399]
    let y: i64 = yoe as i64 + era * 400;
    let doy: u32 = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year  [0, 365]
    let mp: u32 = (5 * doy + 2) / 153; // [0, 11]
    let d: u32 = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m: u32 = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// Returns a list of mounted drive/disk names.
///
/// On Windows the paths are normalised so that a bare drive letter like `C:`
/// becomes `C:\`.
pub fn enumerate_drives() -> Vec<String> {
    // Minimal implementation using std::fs when sysinfo is not available.
    // Returns root paths that are platform-appropriate.
    #[cfg(target_os = "windows")]
    {
        let mut drives = Vec::new();
        // Check common drive letters A-Z
        for letter in b'A'..=b'Z' {
            let path = format!("{}:\\", letter as char);
            if std::path::Path::new(&path).exists() {
                drives.push(path);
            }
        }
        drives
    }

    #[cfg(not(target_os = "windows"))]
    {
        vec!["/".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0.00 B");
        assert_eq!(format_size(500), "500.00 B");
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
        // Values beyond 1024 TB stay in TB
        assert_eq!(format_size(2_199_023_255_552), "2.00 TB");
    }

    #[test]
    fn test_format_number_zero() {
        assert_eq!(format_number(0), "0");
    }

    #[test]
    fn test_format_number_small() {
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
        assert_eq!(ts.len(), 19, "timestamp length should be 19, got: {}", ts);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], " ");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
    }

    #[test]
    fn test_days_to_ymd_epoch() {
        assert_eq!(days_to_ymd(0), (1970, 1, 1));
    }

    #[test]
    fn test_days_to_ymd_known_date() {
        // 2024-01-01 is day 19723
        assert_eq!(days_to_ymd(19_723), (2024, 1, 1));
    }

    #[test]
    fn test_format_system_time_epoch() {
        let epoch = std::time::UNIX_EPOCH;
        assert_eq!(format_system_time(epoch), "1970-01-01 00:00:00");
    }

    #[test]
    fn test_enumerate_drives_not_empty() {
        let drives = enumerate_drives();
        assert!(!drives.is_empty());
    }
}