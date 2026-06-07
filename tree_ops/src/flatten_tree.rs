//! Converts a nested `FsNode` tree to a flat `Vec<UiNode>` for UI rendering.

use std::collections::HashSet;
use std::path::PathBuf;
use crate::models::{FsNode, UiNode};

/// Flattens a nested `FsNode` tree into a `Vec<UiNode>` for UI rendering.
///
/// Only nodes whose ancestors are all present in `expanded_paths` are included
/// (beyond the root level). For nodes with a non-`None` `scan_progress`, the
/// progress value is expressed as a fraction of the largest sibling's size.
///
/// # Arguments
///
/// * `root` - The root `FsNode` to flatten.
/// * `expanded_paths` - Set of paths whose children should be included in output.
/// * `depth` - Starting depth (typically `0` for the root call).
///
/// # Returns
///
/// A `Vec<UiNode>` in pre-order traversal order.
///
/// # Examples
///
///