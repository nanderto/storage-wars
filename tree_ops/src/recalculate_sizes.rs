//! Bottom-up size and file count recalculation for `FsNode` trees.

use crate::models::FsNode;

/// Recalculates sizes and file counts for all directory nodes in the tree
/// by walking bottom-up and summing children's values.
///
/// Leaf nodes (files) retain their existing `size` and `file_count`.
/// Directory nodes have their `size` and `file_count` replaced with the
/// sum of their children's values.
///
/// # Arguments
///
/// * `node` - Mutable reference to the root `FsNode` to recalculate.
///
/// # Examples
///
///