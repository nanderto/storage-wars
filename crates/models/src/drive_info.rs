/// Information about a mounted drive / volume.
#[derive(Clone, Debug, PartialEq)]
pub struct DriveInfo {
    /// OS-level drive name or mount point (e.g. "C:\\").
    pub name: String,
    /// Volume label assigned by the user/OS.
    pub volume_label: String,
    /// Total capacity in bytes.
    pub total_space: u64,
    /// Available (free) space in bytes.
    pub available_space: u64,
}