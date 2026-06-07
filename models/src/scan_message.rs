//! Messages emitted during a scan session.

use serde::{Deserialize, Serialize};

use crate::FsNode;

/// Messages produced by the scanner and consumed by the UI or persistence
/// layer during an active scan session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScanMessage {
    /// Emitted each time a directory has been fully scanned.
    DirScanned {
        /// The node representing the scanned directory.
        node: FsNode,
        /// Number of directories scanned so far in this session.
        dirs_scanned: u64,
    },

    /// Emitted when a non-fatal error is encountered while scanning a path.
    ScanError {
        /// The path that could not be accessed or processed.
        path: String,
        /// A human-readable description of the error.
        message: String,
    },

    /// Emitted once when the entire scan has finished.
    Complete {
        /// Total number of files discovered.
        total_files: u64,
        /// Total number of directories discovered.
        total_dirs: u64,
        /// Aggregate size in bytes of all discovered items.
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

    /// Returns the path associated with a [`ScanMessage::ScanError`], or
    /// `None` for other variants.
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

    #[test]
    fn test_dir_scanned_message() {
        let node = FsNode::new("src", "/project/src");
        let msg = ScanMessage::DirScanned {
            node: node.clone(),
            dirs_scanned: 5,
        };
        assert!(!msg.is_complete());
        assert!(!msg.is_error());
        assert!(msg.error_path().is_none());
    }

    #[test]
    fn test_scan_error_message() {
        let msg = ScanMessage::ScanError {
            path: "/restricted".to_string(),
            message: "Permission denied".to_string(),
        };
        assert!(!msg.is_complete());
        assert!(msg.is_error());
        assert_eq!(msg.error_path(), Some("/restricted"));
    }

    #[test]
    fn test_complete_message() {
        let msg = ScanMessage::Complete {
            total_files: 1000,
            total_dirs: 50,
            total_size: 1_073_741_824,
            error_count: 2,
        };
        assert!(msg.is_complete());
        assert!(!msg.is_error());
        assert!(msg.error_path().is_none());
    }

    #[test]
    fn test_serialization_roundtrip_dir_scanned() {
        let node = FsNode::new("bin", "/usr/bin");
        let msg = ScanMessage::DirScanned {
            node,
            dirs_scanned: 12,
        };
        let json = serde_json::to_string(&msg).expect("serialization failed");
        let restored: ScanMessage = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(msg, restored);
    }

    #[test]
    fn test_serialization_roundtrip_complete() {
        let msg = ScanMessage::Complete {
            total_files: 500,
            total_dirs: 20,
            total_size: 524_288_000,
            error_count: 0,
        };
        let json = serde_json::to_string(&msg).expect("serialization failed");
        let restored: ScanMessage = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(msg, restored);
    }
}