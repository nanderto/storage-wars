/// Metadata for a single scan session.
#[derive(Clone, Debug, PartialEq)]
pub struct ScanMeta {
    /// Unique identifier for the scan.
    pub id: i64,
    /// Drive path or identifier that was scanned (e.g. "C:\\").
    pub drive: String,
    /// ISO-8601 timestamp of when the scan was performed.
    pub timestamp: String,
    /// Total size in bytes observed during the scan.
    pub total_size: u64,
}