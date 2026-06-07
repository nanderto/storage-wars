//! Converts a nested [`FsNode`] tree to a flat `Vec<UiNode>` for UI rendering.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::types::{FsNode, UiNode};

/// Flattens a slice of root [`FsNode`]s into a `Vec<UiNode>` suitable for UI rendering.
///
/// Only nodes whose ancestors are all present in `expanded_paths` are included
/// (root nodes are always included). The `scan_progress` field is computed as the
/// ratio of a node's `size` to the maximum sibling `size` within the same parent,
/// giving a value in `[0.0, 1.0]`.
///
/// # Arguments
///
/// * `roots` — The root nodes of the filesystem tree.
/// * `expanded_paths` — Set of paths that are currently expanded in the UI.
///
/// # Returns
///
/// A depth-first ordered `Vec<UiNode>` containing all visible nodes.
///
/// # Examples
///
///