/// Describes a size delta between two scans, with a display colour.
#[derive(Clone, Debug, PartialEq)]
pub struct SizeChange {
    /// Absolute size difference in bytes.
    pub delta: i64,
    /// Human-readable classification, e.g. "increased", "decreased", "unchanged".
    pub classification: String,
    /// Hex colour string for UI rendering (e.g. "#FF0000").
    pub hex_color: String,
}