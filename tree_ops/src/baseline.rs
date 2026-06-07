//! Baseline snapshot utilities: building a path→size map and merging it into a tree.

use std::collections::HashMap;
use std::path::PathBuf;
use crate::models::{BaselineMap, FsNode};

/// Builds a `BaselineMap` (path → size) from a slice of `FsNode` trees.
///
/// All nodes in the forest are visited recursively, and each node's path and
/// size are inserted into the map.
///
/// # Arguments
///
/// * `roots` - Slice of root `FsNode` trees to traverse.
///
/// # Returns
///
/// A `HashMap<PathBuf, u64>` mapping each node's path to its size.
///
/// # Examples
///
///