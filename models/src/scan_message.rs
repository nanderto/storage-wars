//! Messages emitted during a scan session.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Messages produced by the scanner and consumed by the UI or persistence layer.
///
/// These messages form the communication channel between the background scan
/// worker and the rest of the application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanMessage {
    /// Emitted each time a directory has been fully scanned.
    DirScanned {
        /// The node representing the scanned directory and its immediate children.
        node: FsNode,
        /// Number of entries scanned so far in the current session.
        entries_scanned: u64,
    },

    /// Emitted when a non-fatal error is encountered while scanning an entry.
    ScanError {
        /// The path that caused the error.
        path: String,
        /// Human-readable description of the error.
        message: String,
    },

    /// Emitted once when the entire scan has finished.
    Complete {
        /// Total number of files found.
        total_files: u64,
        /// Total number of folders found.
        total_folders: u64,
        /// Total size in bytes of all scanned entries.
        total_size: u64,
        /// Whether the scan completed without any errors.
        had_errors: bool,
    },
}

impl ScanMessage {
    /// Returns `true` if this message signals scan completion.
    pub fn is_complete(&self) -> bool {
        matches!(self, ScanMessage::Complete { .. })
    }

    /// Returns `true` if this message represents an error.
    pub fn is_error(&self) -> bool {
        matches!(self, ScanMessage::ScanError { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_message_is_complete() {
        let msg = ScanMessage::Complete {
            total_files: 100,
            total_folders: 10,
            total_size: 1_048_576,
            had_errors: false,
        };
        assert!(msg.is_complete());
        assert!(!msg.is_error());
    }

    #[test]
    fn error_message_is_error() {
        let msg = ScanMessage::ScanError {
            path: "/some/path".to_string(),
            message: "Permission denied".to_string(),
        };
        assert!(msg.is_error());
        assert!(!msg.is_complete());
    }

    #[test]
    fn dir_scanned_is_neither_complete_nor_error() {
        let node = FsNode::new_dir("src", "/project/src");
        let msg = ScanMessage::DirScanned {
            node,
            entries_scanned: 42,
        };
        assert!(!msg.is_complete());
        assert!(!msg.is_error());
    }
}