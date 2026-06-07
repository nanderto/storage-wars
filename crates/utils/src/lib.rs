/// Formats a byte count into a human-readable string with 2 decimal places.
///
/// Uses units: B, KB, MB, GB, TB (base-1024).
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut size = bytes as f64;
    for &unit in UNITS {
        if size < 1024.0 || unit == "TB" {
            return if unit == "B" {
                format!("{} {}", bytes, unit)
            } else {
                format!("{:.2} {}", size, unit)
            };
        }
        size /= 1024.0;
    }
    unreachable!()
}

/// Formats a u64 number with comma thousand-separators.
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len <= 3 {
        return s;
    }
    let mut result = String::with_capacity(len + (len - 1) / 3);
    let first_group = len % 3;
    if first_group > 0 {
        for &b in &bytes[..first_group] {
            result.push(b as char);
        }
    }
    for (i, chunk) in bytes[first_group..].chunks(3).enumerate() {
        if i > 0 || first_group > 0 {
            result.push(',');
        }
        for &b in chunk {
            result.push(b as char);
        }
    }
    result
}

/// Converts a day count (from Unix epoch, March-based) into (year, month, day)
/// using the Hinnant algorithm.
fn days_to_ymd(days: i64) -> (i64, u32, u32) {
    // Civil days from Unix epoch to internal epoch (2000-03-01)
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // day of era [0, 146096]
    let yoe =
        (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // year of era [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
    let mp = (5 * doy + 2) / 153; // month index [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // day [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // month [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as u32, d as u32)
}

/// Returns the current UTC timestamp as a string in `YYYY-MM-DD HH:MM:SS` format.
///
/// Uses `std::time::SystemTime` — no external crate required.
pub fn chrono_now() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before Unix epoch");

    let total_secs = dur.as_secs();
    let secs_in_day: u64 = 86400;
    let days = (total_secs / secs_in_day) as i64;
    let remaining = total_secs % secs_in_day;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    let (year, month, day) = days_to_ymd(days);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

/// Converts a `std::time::SystemTime` to an ISO 8601 UTC string (`YYYY-MM-DD HH:MM:SS`).
pub fn format_system_time(time: std::time::SystemTime) -> String {
    let dur = time
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();

    let total_secs = dur.as_secs();
    let secs_in_day: u64 = 86400;
    let days = (total_secs / secs_in_day) as i64;
    let remaining = total_secs % secs_in_day;
    let hours = remaining / 3600;
    let minutes = (remaining % 3600) / 60;
    let seconds = remaining % 60;

    let (year, month, day) = days_to_ymd(days);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_zero() {
        assert_eq!(format_size(0), "0 B");
    }

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(500), "500 B");
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
        assert_eq!(format_number(1_234_567), "1,234,567");
        assert_eq!(format_number(1_000_000_000), "1,000,000,000");
    }

    #[test]
    fn test_chrono_now_format() {
        let ts = chrono_now();
        // Should match YYYY-MM-DD HH:MM:SS
        assert_eq!(ts.len(), 19);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], " ");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
    }

    #[test]
    fn test_days_to_ymd_epoch() {
        // Day 0 = 1970-01-01
        let (y, m, d) = days_to_ymd(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn test_days_to_ymd_known_date() {
        // 2024-01-01 is day 19723 from Unix epoch
        let (y, m, d) = days_to_ymd(19723);
        assert_eq!((y, m, d), (2024, 1, 1));
    }

    #[test]
    fn test_format_system_time_epoch() {
        let epoch = std::time::UNIX_EPOCH;
        assert_eq!(format_system_time(epoch), "1970-01-01 00:00:00");
    }
}