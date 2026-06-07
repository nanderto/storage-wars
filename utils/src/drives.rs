//! Drive enumeration using the `sysinfo` crate.
//!
//! On Windows, drive mount points are normalised to the familiar
//! `C:\` letter format.  On other platforms the raw mount point is returned.

use sysinfo::Disks;

/// Returns a list of drive/disk mount points detected on the current system.
///
/// ## Windows normalisation
///
/// `sysinfo` may return mount points such as `\\?\C:\` on Windows.
/// This function strips the `\\?\` prefix so callers receive the familiar
/// `C:\` format.
///
/// ## Examples
///
///