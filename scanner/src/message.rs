//! Messages sent from scanner worker threads to the UI layer.

use std::path::PathBuf;

use crate::scanner::DirNode;

/// Messages produced by [`crate::scan_dir_incremental`] and forwarded to the UI.
#[derive(Debug)]
pub enum ScanMessage {
    /// A directory has been fully scanned.
    ///
    /// Contains the [`DirNode`] with aggregated size and immediate children.
    DirScanned(DirNode),

    /// A non-fatal error occurred while scanning `path`.
    ///
    /// The scan continues; permission errors are silently skipped and never
    /// produce this variant.
    ScanError {
        path: PathBuf,
        error: String,
    },

    /// The entire scan has finished (either completed or cancelled).
    ///
    /// After receiving this message the receiver can drop the channel.
    Complete,
}