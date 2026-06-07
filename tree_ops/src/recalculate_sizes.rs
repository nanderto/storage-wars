//! Bottom-up size and file-count aggregation for [`FsNode`] trees.

use crate::models::FsNode;

/// Walks the tree bottom-up, summing `size` and `file_count` from children
/// into each parent node.
///
/// Leaf nodes (files) retain their original `size` and contribute `1` to
/// `file_count`. Directory nodes receive the sum of their children's values.
///
/// # Arguments
///
/// * `nodes` — Mutable slice of root nodes to recalculate in place.
///
/// # Examples
///
///