//! Baseline map construction and merging for size comparison across scans.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::{FsNode, UiNode};

/// Builds a `PathBuf → u64` lookup map from a slice of [`FsNode`]s.
///
/// Traverses the entire tree (all descendants) and records each node's path and size.
///
/// # Arguments
///
/// * `nodes` — A slice of root [`FsNode`]s representing the baseline scan.
///
/// # Returns
///
/// A `HashMap<PathBuf, u64>` mapping each node's path to its size.
///
/// # Examples
///
///