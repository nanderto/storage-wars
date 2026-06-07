//! Error types for the scanner component.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Errors that can occur during filesystem scanning.
#[derive(Debug)]
pub enum ScanError {
    /// An I/O error occurred while accessing a path.
    Io {
        path: PathBuf,
        source: io::Error,
    },
    /// The scan was cancelled via the atomic flag before completion.
    Cancelled,
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::Io { path, source } => {
                write!(f, "I/O error at '{}': {}", path.display(), source)
            }
            ScanError::Cancelled => write!(f, "scan was cancelled"),
        }
    }
}

impl std::error::Error for ScanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ScanError::Io { source, .. } => Some(source),
            ScanError::Cancelled => None,
        }
    }
}

impl ScanError {
    /// Returns `true` if this error represents a permission denial.
    pub fn is_permission_denied(&self) -> bool {
        match self {
            ScanError::Io { source, .. } => source.kind() == io::ErrorKind::PermissionDenied,
            ScanError::Cancelled => false,
        }
    }
}