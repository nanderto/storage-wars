//! Converts a nested [`FsNode`] tree to a flat `Vec<UiNode>` for UI rendering.

use crate::models::{ExpandedPaths, FsNode, UiNode};

/// Flattens a forest of [`FsNode`] trees into a `Vec<UiNode>` for UI rendering.
///
/// Only children of nodes whose paths appear in `expanded_paths` are included.
/// Each node's `size_fraction` is computed as a fraction of the largest sibling's
/// size at the same level (or `scan_progress` if the node is still being scanned).
///
/// # Arguments
///
/// * `roots`          — Root nodes of the filesystem tree.
/// * `expanded_paths` — Set of directory paths that are currently expanded.
///
/// # Returns
///
/// A depth-first, pre-order `Vec<UiNode>` ready for rendering.
///
/// # Examples
///
///