/// Messages emitted by the scanner during a scan session.
#[derive(Clone, Debug, PartialEq)]
pub enum ScanMessage {
    /// A directory has been scanned; carries the path and cumulative byte count.
    DirScanned {
        path: String,
        bytes_so_far: u64,
    },
    /// A non-fatal error occurred while scanning.
    ScanError {
        path: String,
        message: String,
    },
    /// The scan has finished successfully.
    Complete {
        total_size: u64,
    },
}