//! Baseline map utilities for tracking size changes between scans.

use std::collections::HashMap;
use std::path::PathBuf;
use crate::models::FsNode;

/// A lookup map from filesystem path to the previously recorded size in bytes.
pub type BaselineMap = HashMap<PathBuf, u64>;

/// Builds a [`BaselineMap`] from the current state of the tree.
///
/// Walks the entire tree and records each node's `path → size` pair.
/// This snapshot can later be passed to [`merge_baseline`] to populate
/// `prev_size` on a freshly scanned tree.
///
/// # Arguments
///
/// * `roots` — Root nodes of the filesystem tree to snapshot.
///
/// # Returns
///
/// A `HashMap<PathBuf, u64>` mapping each node's path to its current size.
///
/// # Examples
///
///