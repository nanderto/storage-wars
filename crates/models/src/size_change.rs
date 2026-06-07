/// Classifies the delta between current and previous size.
#[derive(Clone, Debug, PartialEq)]
pub struct SizeChange {
    /// Absolute size difference in bytes.
    pub delta: i64,
    /// Human-readable label (e.g. "+2.3 MB", "-512 KB").
    pub label: String,
    /// Hex colour code for UI rendering (e.g. "#FF0000" for growth).
    pub color: String,
}