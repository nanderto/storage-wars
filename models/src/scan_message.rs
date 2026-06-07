//! Messages emitted during an active scan session.

use serde::{Deserialize, Serialize};

use crate::fs_node::FsNode;

/// Messages produced by the scanner and consumed by the UI or persistence
/// layer during an active scan session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanMessage {
    /// A directory has been fully scanned.
    ///
    /// Contains the [`FsNode`] representing the scanned directory and the
    /// total number of items processed so far.
    DirScanned {
        /// The fully populated node for the scanned directory.
        node: FsNode,
        /// Running count of items (files + directories) processed so far.
        items_processed: u64,
    },

    /// A non-fatal error occurred while scanning a path.
    ///
    /// The scan continues after emitting this message.
    ScanError {
        /// The path that could not be accessed or processed.
        path: String,
        /// Human-readable description of the error.
        message: String,
    },

    /// The scan session has finished.
    ///
    /// Contains the root [`FsNode`] of the fully scanned tree and a flag
    /// indicating whether the scan completed without any errors.
    Complete {
        /// The root node of the completed scan tree.
        root: FsNode,
        /// `true` if no errors were encountered during the scan.
        success: bool,
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

    fn dummy_node() -> FsNode {
        FsNode::new("root", "/", true)
    }

    #[test]
    fn is_complete_returns_true_for_complete_variant() {
        let msg = ScanMessage::Complete {
            root: dummy_node(),
            success: true,
        };
        assert!(msg.is_complete());
        assert!(!msg.is_error());
    }

    #[test]
    fn is_error_returns_true_for_scan_error_variant() {
        let msg = ScanMessage::ScanError {
            path: "/locked".to_string(),
            message: "Permission denied".to_string(),
        };
        assert!(msg.is_error());
        assert!(!msg.is_complete());
    }

    #[test]
    fn dir_scanned_is_neither_complete_nor_error() {
        let msg = ScanMessage::DirScanned {
            node: dummy_node(),
            items_processed: 42,
        };
        assert!(!msg.is_complete());
        assert!(!msg.is_error());
    }
}