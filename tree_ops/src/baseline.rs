//! Baseline map construction and merging for size change detection.

use std::collections::HashMap;
use std::path::PathBuf;
use crate::models::{FsNode, UiNode};

/// Builds a `PathBuf → u64` lookup map from a slice of `FsNode` records.
///
/// The map associates each node's path with its size, enabling efficient
/// baseline comparisons after a re-scan.
///
/// # Arguments
///
/// * `nodes` - A flat slice of `FsNode` records to index.
///
/// # Returns
///
/// A `HashMap<PathBuf, u64>` mapping each path to its size.
///
/// # Examples
///
///