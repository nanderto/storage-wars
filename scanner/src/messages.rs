//! Message types sent from scanner workers to the UI layer.

use crate::models::DirEntry;
use std::path::PathBuf;

/// Messages produced by the scanner and consumed by the UI or caller.
#[derive(Debug)]
pub enum ScanMessage {
    /// A directory has been fully scanned and its aggregated size is known.
    DirScanned(DirEntry),

    /// A non-fatal error occurred while scanning a path.
    ScanError {
        /// The path that caused the error.
        path: PathBuf,
        /// Human-readable description of the error.
        error: String,
    },

    /// The scan has finished (either fully or due to cancellation).
    Complete,
}