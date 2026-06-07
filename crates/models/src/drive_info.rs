/// Information about a mounted drive / volume.
#[derive(Clone, Debug, PartialEq)]
pub struct DriveInfo {
    /// Drive name / mount point (e.g. "C:\\" or "/mnt/data").
    pub name: String,
    /// Volume label reported by the OS, if any.
    pub volume_label: String,
    /// Total capacity of the drive in bytes.
    pub total_space: u64,
    /// Available (free) space on the drive in bytes.
    pub available_space: u64,
}