//! Converts a nested `FsNode` tree into a flat `Vec<UiNode>` for UI rendering.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::types::{FsNode, UiNode};

/// Flattens a nested `FsNode` tree into a `Vec<UiNode>` for UI rendering.
///
/// Only nodes whose parent paths are present in `expanded_paths` are included
/// beyond the root level. The `scan_progress` field of each `UiNode` is set
/// to the node's size divided by the maximum sibling size (or `1.0` if there
/// is only one sibling or the maximum is zero).
///
/// # Arguments
///
/// * `roots` - The root nodes of the `FsNode` hierarchy.
/// * `expanded_paths` - The set of paths whose children should be rendered.
///
/// # Returns
///
/// A flat `Vec<UiNode>` in pre-order (depth-first) traversal order.
///
/// # Examples
///
///