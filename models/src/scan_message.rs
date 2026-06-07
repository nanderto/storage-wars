//! Messages emitted during a scan session.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Messages produced by the scanner and consumed by the UI or persistence layer.
///
/// These messages are typically sent over a channel from a background scan thread
/// to the main application thread.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanMessage {
    /// Emitted each time a directory has been fully scanned.
    DirScanned {
        /// The scanned directory node, including its immediate children.
        node: FsNode,
        /// Total number of items scanned so far in this session.
        items_scanned: u64,
    },

    /// Emitted when a non-fatal error is encountered while scanning a path.
    ScanError {
        /// The path that caused the error.
        path: String,
        /// A human-readable description of the error.
        message: String,
    },

    /// Emitted once when the entire scan has finished.
    Complete {
        /// Total number of files found.
        total_files: u64,
        /// Total number of folders found.
        total_folders: u64,
        /// Total size in bytes of all scanned items.
        total_size: u64,
        /// Number of non-fatal errors encountered during the scan.
        error_count: u64,
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

    /// Returns the path associated with a `ScanError` message, or `None` for other variants.
    pub fn error_path(&self) -> Option<&str> {
        match self {
            ScanMessage::ScanError { path, .. } => Some(path.as_str()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FsNode;

    #[test]
    fn test_is_complete() {
        let msg = ScanMessage::Complete {
            total_files: 100,
            total_folders: 10,
            total_size: 1_000_000,
            error_count: 0,
        };
        assert!(msg.is_complete());
        assert!(!msg.is_error());
    }

    #[test]
    fn test_is_error() {
        let msg = ScanMessage::ScanError {
            path: "/restricted".to_string(),
            message: "Permission denied".to_string(),
        };
        assert!(msg.is_error());
        assert!(!msg.is_complete());
        assert_eq!(msg.error_path(), Some("/restricted"));
    }

    #[test]
    fn test_dir_scanned_not_complete_or_error() {
        let node = FsNode::new_dir("home", "/home", None);
        let msg = ScanMessage::DirScanned {
            node,
            items_scanned: 42,
        };
        assert!(!msg.is_complete());
        assert!(!msg.is_error());
        assert!(msg.error_path().is_none());
    }
}