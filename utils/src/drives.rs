//! Drive enumeration using the `sysinfo` crate.
//!
//! On Windows, drive mount points are normalized to the conventional
//! `C:\` letter format.

use sysinfo::Disks;

/// Returns a list of drive/disk mount-point strings detected on the current system.
///
/// On Windows, mount points such as `\\?\Volume{...}\` are replaced with the
/// conventional drive-letter form (e.g. `C:\`) when `sysinfo` exposes them.
/// Paths that already look like `C:\` are left unchanged.
///
/// # Examples
///
///