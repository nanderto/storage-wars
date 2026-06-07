//! Bottom-up size and count recalculation for `FsNode` trees.

use crate::types::FsNode;

/// Walks the `FsNode` tree bottom-up, recomputing `size` and `child_count`
/// for every directory node from its children.
///
/// Leaf nodes (files) retain their original `size` and `child_count` values.
/// Directory nodes have their `size` set to the sum of all direct children's
/// sizes, and their `child_count` set to the total number of all descendants.
///
/// # Arguments
///
/// * `nodes` - A mutable slice of root `FsNode` entries to recalculate in place.
///
/// # Examples
///
///