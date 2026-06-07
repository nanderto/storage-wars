//! Scan event messages communicated between the scanner and the UI.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Messages emitted by the background scanner to report progress and results.
///
/// These messages are typically sent over a channel (e.g. `std::sync::mpsc`) from
/// the scanning thread to the UI thread.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanMessage {
    /// A directory has been fully scanned.
    ///
    /// Contains the [`FsNode`] representing the scanned directory, including
    /// aggregated size and counts for all its immediate children.
    DirScanned {
        /// The scanned directory node.
        node: FsNode,
        /// The total number of items scanned so far in this session.
        items_scanned: u64,
    },

    /// A non-fatal error occurred while scanning a specific path.
    ///
    /// The scan continues after a [`ScanMessage::ScanError`].
    ScanError {
        /// The path that could not be accessed.
        path: String,
        /// A human-readable description of the error.
        message: String,
    },

    /// The scan has completed (successfully or after encountering errors).
    Complete {
        /// The total number of files scanned.
        total_files: u64,
        /// The total number of folders scanned.
        total_folders: u64,
        /// The total size in bytes of all scanned items.
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

    /// Returns `true` if this message is an error notification.
    pub fn is_error(&self) -> bool {
        matches!(self, ScanMessage::ScanError { .. })
    }

    /// Returns `true` if this message carries a scanned directory node.
    pub fn is_dir_scanned(&self) -> bool {
        matches!(self, ScanMessage::DirScanned { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_node() -> FsNode {
        FsNode::new_dir("home", "/home")
    }

    #[test]
    fn dir_scanned_is_recognized() {
        let msg = ScanMessage::DirScanned {
            node: sample_node(),
            items_scanned: 42,
        };
        assert!(msg.is_dir_scanned());
        assert!(!msg.is_complete());
        assert!(!msg.is_error());
    }

    #[test]
    fn scan_error_is_recognized() {
        let msg = ScanMessage::ScanError {
            path: "/restricted".to_string(),
            message: "Permission denied".to_string(),
        };
        assert!(msg.is_error());
        assert!(!msg.is_complete());
        assert!(!msg.is_dir_scanned());
    }

    #[test]
    fn complete_is_recognized() {
        let msg = ScanMessage::Complete {
            total_files: 1000,
            total_folders: 50,
            total_size: 5_000_000,
            had_errors: false,
        };
        assert!(msg.is_complete());
        assert!(!msg.is_error());
        assert!(!msg.is_dir_scanned());
    }

    #[test]
    fn serialization_round_trip_dir_scanned() {
        let msg = ScanMessage::DirScanned {
            node: sample_node(),
            items_scanned: 10,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: ScanMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, restored);
    }

    #[test]
    fn serialization_round_trip_complete() {
        let msg = ScanMessage::Complete {
            total_files: 500,
            total_folders: 25,
            total_size: 1_024_000,
            had_errors: true,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: ScanMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, restored);
    }
}