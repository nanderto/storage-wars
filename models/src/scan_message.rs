//! Messages emitted during a filesystem scan.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Represents a message produced by the scanner during a scan session.
///
/// Consumers (e.g. the UI or a database writer) receive a stream of these
/// messages and react accordingly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanMessage {
    /// Emitted each time a directory has been fully scanned.
    DirScanned {
        /// The directory node that was just scanned, including its children.
        node: FsNode,
        /// The total number of items scanned so far in this session.
        items_scanned: u64,
    },

    /// Emitted when a non-fatal error occurs while scanning a path.
    ScanError {
        /// The path that caused the error.
        path: String,
        /// A human-readable description of the error.
        message: String,
    },

    /// Emitted once when the entire scan has finished.
    Complete {
        /// The root node of the fully-scanned tree.
        root: FsNode,
        /// Total number of files found.
        total_files: u64,
        /// Total number of folders found.
        total_folders: u64,
        /// Total size in bytes of all scanned items.
        total_size: u64,
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

    /// Returns the path associated with a [`ScanMessage::ScanError`], if any.
    pub fn error_path(&self) -> Option<&str> {
        if let ScanMessage::ScanError { path, .. } = self {
            Some(path.as_str())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_node() -> FsNode {
        FsNode::new_dir("root", "/", None)
    }

    #[test]
    fn complete_message_is_complete() {
        let msg = ScanMessage::Complete {
            root: dummy_node(),
            total_files: 10,
            total_folders: 3,
            total_size: 4096,
        };
        assert!(msg.is_complete());
        assert!(!msg.is_error());
    }

    #[test]
    fn error_message_is_error() {
        let msg = ScanMessage::ScanError {
            path: "/bad/path".to_string(),
            message: "Permission denied".to_string(),
        };
        assert!(msg.is_error());
        assert!(!msg.is_complete());
        assert_eq!(msg.error_path(), Some("/bad/path"));
    }

    #[test]
    fn dir_scanned_is_neither_complete_nor_error() {
        let msg = ScanMessage::DirScanned {
            node: dummy_node(),
            items_scanned: 5,
        };
        assert!(!msg.is_complete());
        assert!(!msg.is_error());
    }
}