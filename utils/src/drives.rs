//! Drive enumeration using `sysinfo`.

use sysinfo::Disks;

/// Represents a single drive/disk with its mount point and storage metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct DriveInfo {
    /// Normalized drive identifier.
    ///
    /// On Windows, single-character drive letters are uppercased and suffixed with `:\`
    /// (e.g. `"C:\"`, `"D:\"`).  On other platforms the raw mount-point string is used.
    pub name: String,
    /// Total disk capacity in bytes.
    pub total_bytes: u64,
    /// Available (free) disk space in bytes.
    pub available_bytes: u64,
}

/// Enumerates all disks visible to the OS and returns a [`Vec<DriveInfo>`].
///
/// Drive letter normalization (Windows):
/// - Mount points that are a single ASCII letter are expanded to `X:\` form.
/// - The letter is uppercased.
///
/// # Examples
///
///