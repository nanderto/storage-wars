//! Drive enumeration using the `sysinfo` crate.
//!
//! On Windows, drive mount points are normalized to the conventional
//! `C:\` letter format. On other platforms the raw mount point is returned.

use sysinfo::Disks;

/// Returns a list of drive/disk mount points available on the system.
///
/// On Windows, paths like `\\?\Volume{...}\` or `/C:` are normalized to
/// the conventional `C:\` drive-letter format when possible.
///
/// # Examples
///