//! Messages emitted during a scan session.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Messages produced by the scanner and consumed by the UI or storage layer.
///
/// These messages form the communication protocol between the background
/// scanning thread and the rest of the application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanMessage {
    /// Emitted each time a directory has been fully scanned.
    DirScanned {
        /// The scanned directory node, including its immediate children.
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
        /// The root node of the fully scanned tree.
        root: FsNode,
        /// Total number of files found.
        total_files: u64,
        /// Total number of folders found.
        total_folders: u64,
        /// Total size in bytes of all scanned items.
        total_size: u64,
        /// Number of errors encountered during the scan.
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

    /// Returns the path associated with this message, if any.
    pub fn path(&self) -> Option<&str> {
        match self {
            ScanMessage::DirScanned { node, .. } => Some(&node.path),
            ScanMessage::ScanError { path, .. } => Some(path),
            ScanMessage::Complete { root, .. } => Some(&root.path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_node() -> FsNode {
        FsNode::new_dir("root", "/")
    }

    #[test]
    fn test_dir_scanned_message() {
        let msg = ScanMessage::DirScanned {
            node: sample_node(),
            items_scanned: 42,
        };
        assert!(!msg.is_complete());
        assert!(!msg.is_error());
        assert_eq!(msg.path(), Some("/"));
    }

    #[test]
    fn test_scan_error_message() {
        let msg = ScanMessage::ScanError {
            path: "/restricted".to_string(),
            message: "Permission denied".to_string(),
        };
        assert!(msg.is_error());
        assert!(!msg.is_complete());
        assert_eq!(msg.path(), Some("/restricted"));
    }

    #[test]
    fn test_complete_message() {
        let msg = ScanMessage::Complete {
            root: sample_node(),
            total_files: 1000,
            total_folders: 50,
            total_size: 5_000_000,
            error_count: 2,
        };
        assert!(msg.is_complete());
        assert!(!msg.is_error());
        assert_eq!(msg.path(), Some("/"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let msg = ScanMessage::DirScanned {
            node: sample_node(),
            items_scanned: 10,
        };
        let json = serde_json::to_string(&msg).expect("serialization failed");
        let restored: ScanMessage = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(msg, restored);
    }

    #[test]
    fn test_error_serialization_roundtrip() {
        let msg = ScanMessage::ScanError {
            path: "/foo/bar".to_string(),
            message: "IO error".to_string(),
        };
        let json = serde_json::to_string(&msg).expect("serialization failed");
        let restored: ScanMessage = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(msg, restored);
    }
}