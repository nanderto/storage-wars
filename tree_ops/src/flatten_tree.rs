//! Converts a nested `FsNode` hierarchy to a flat `Vec<UiNode>` for UI rendering.

use std::collections::HashSet;
use std::path::PathBuf;
use crate::models::{FsNode, UiNode};

/// Flattens a slice of root `FsNode` trees into a `Vec<UiNode>` for UI rendering.
///
/// Only nodes whose ancestors are all present in `expanded_paths` are included
/// (i.e., the tree is traversed depth-first but children are only visited when
/// the parent path is expanded). Root nodes are always included.
///
/// `scan_progress` for each node is computed as `node.size / max_sibling_size`
/// where `max_sibling_size` is the largest size among siblings at the same level.
/// If all siblings have size 0, `scan_progress` defaults to `0.0`.
///
/// # Arguments
///
/// * `roots` - Slice of root `FsNode` trees to flatten.
/// * `expanded_paths` - Set of paths that are currently expanded in the UI.
///
/// # Returns
///
/// A `Vec<UiNode>` in depth-first pre-order traversal order.
///
/// # Examples
///
///