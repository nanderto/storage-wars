//! Bottom-up size and count recalculation for the filesystem tree.

use crate::types::FsNode;

/// Walks the tree bottom-up, recalculating `size` and `file_count` for every node.
///
/// For leaf nodes (files), `size` and `file_count` are left unchanged.
/// For directory nodes, `size` is set to the sum of all children's sizes and
/// `file_count` is set to the sum of all children's `file_count` values.
///
/// # Arguments
///
/// * `nodes` — A mutable slice of root [`FsNode`]s to recalculate.
///
/// # Examples
///
///