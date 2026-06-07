/// Messages emitted during a scan operation.
#[derive(Clone, Debug, PartialEq)]
pub enum ScanMessage {
    /// A directory has been fully scanned. Contains the path and cumulative size.
    DirScanned {
        path: String,
        size: u64,
    },
    /// A non-fatal error occurred while scanning a path.
    ScanError {
        path: String,
        message: String,
    },
    /// The scan has finished. Contains the total size observed.
    Complete {
        total_size: u64,
    },
}