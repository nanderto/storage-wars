/// Information about a mounted drive / volume.
#[derive(Clone, Debug, PartialEq)]
pub struct DriveInfo {
    /// Mount point or drive letter path (e.g. "C:\\").
    pub name: String,
    /// Volume label reported by the OS (may be empty).
    pub volume_label: String,
    /// Total capacity in bytes.
    pub total_space: u64,
    /// Available (free) space in bytes.
    pub available_space: u64,
}