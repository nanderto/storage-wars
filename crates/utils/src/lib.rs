use chrono::Local;

/// Formats a byte count into a human-readable string with 2 decimal places.
///
/// Uses units: B, KB, MB, GB, TB.
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const SCALE: f64 = 1024.0;

    if bytes == 0 {
        return "0 B".to_string();
    }

    let bytes_f = bytes as f64;
    let exp = (bytes_f.ln() / SCALE.ln()).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);

    let value = bytes_f / SCALE.powi(exp as i32);

    if exp == 0 {
        format!("{} {}", bytes, UNITS[exp])
    } else {
        format!("{:.2} {}", value, UNITS[exp])
    }
}

/// Formats a number with comma thousand-separators.
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let len = s.len();

    if len <= 3 {
        return s;
    }

    let mut result = String::with_capacity(len + (len - 1) / 3);
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }

    result
}

/// Returns the current local timestamp formatted as `YYYY-MM-DD HH:MM:SS`.
pub fn chrono_now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
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
    fn test_format_size_kilobytes() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
    }

    #[test]
    fn test_format_size_megabytes() {
        assert_eq!(format_size(1_048_576), "1.00 MB");
    }

    #[test]
    fn test_format_size_gigabytes() {
        assert_eq!(format_size(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn test_format_size_terabytes() {
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
        assert_eq!(format_number(123_456), "123,456");
    }

    #[test]
    fn test_format_number_millions() {
        assert_eq!(format_number(1_000_000), "1,000,000");
        assert_eq!(format_number(1_234_567_890), "1,234,567,890");
    }

    #[test]
    fn test_chrono_now_format() {
        let now = chrono_now();
        // Verify the format: YYYY-MM-DD HH:MM:SS (19 chars)
        assert_eq!(now.len(), 19);
        assert_eq!(&now[4..5], "-");
        assert_eq!(&now[7..8], "-");
        assert_eq!(&now[10..11], " ");
        assert_eq!(&now[13..14], ":");
        assert_eq!(&now[16..17], ":");
    }
}