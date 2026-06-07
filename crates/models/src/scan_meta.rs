/// Metadata for a single scan session.
#[derive(Clone, Debug, PartialEq)]
pub struct ScanMeta {
    /// Unique identifier for this scan.
    pub id: i64,
    /// Drive path that was scanned (e.g. "C:\\" or "/").
    pub drive: String,
    /// ISO-8601 timestamp of when the scan was performed.
    pub timestamp: String,
    /// Total size in bytes discovered during the scan.
    pub total_size: u64,
}