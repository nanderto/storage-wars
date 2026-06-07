//! Baseline snapshot utilities: building a lookup map and merging into the tree.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::FsNode;

/// Builds a `PathBuf → u64` lookup map from a slice of `FsNode` records.
///
/// The map contains one entry per node, mapping its path to its size.
/// This is typically called on a previous snapshot of the tree to create
/// a baseline for comparison.
///
/// # Arguments
///
/// * `nodes` - A flat slice of `FsNode` records (need not be a tree).
///
/// # Returns
///
/// A `HashMap<PathBuf, u64>` mapping each node's path to its size.
///
/// # Examples
///
///