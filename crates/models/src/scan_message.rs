use std::path::PathBuf;

/// Messages emitted by the scanner during a scan session.
#[derive(Clone, Debug, PartialEq)]
pub enum ScanMessage {
    /// A directory has been successfully scanned.
    DirScanned {
        /// Path of the directory that was scanned.
        path: PathBuf,
        /// Cumulative number of files discovered so far.
        file_count: u64,
    },
    /// An error occurred while scanning a particular path.
    ScanError {
        /// Path where the error occurred.
        path: PathBuf,
        /// Human-readable error message.
        message: String,
    },
    /// The scan session has completed.
    Complete {
        /// Total size in bytes of all discovered entries.
        total_size: u64,
    },
}