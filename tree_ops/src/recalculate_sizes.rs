//! Bottom-up size and item count recalculation for `FsNode` trees.

use crate::models::FsNode;

/// Recalculates sizes and item counts for all nodes in a forest by walking bottom-up.
///
/// For each directory node, `size` is set to the sum of all children's sizes,
/// and `item_count` is set to the total number of descendant leaf (non-directory) nodes.
/// Leaf nodes (files) retain their original `size` and have `item_count = 1`.
///
/// # Arguments
///
/// * `roots` - Mutable slice of root `FsNode` trees to recalculate.
///
/// # Examples
///
///