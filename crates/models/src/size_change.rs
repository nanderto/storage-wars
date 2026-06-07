/// Represents how a node's size changed between two scans.
#[derive(Clone, Debug, PartialEq)]
pub struct SizeChange {
    /// Absolute size delta in bytes (positive = growth, negative = shrink).
    pub delta: i64,
    /// Human-readable classification, e.g. "increased", "decreased", "unchanged".
    pub classification: String,
    /// Hex color string for UI display (e.g. "#FF0000" for growth).
    pub hex_color: String,
}