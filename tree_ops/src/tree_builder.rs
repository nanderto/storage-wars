//! Reconstructs an `FsNode` hierarchy from a flat list of `DbNode` records.

use std::collections::HashMap;

use crate::types::{DbNode, FsNode};

/// Builds a nested `FsNode` tree from a flat list of [`DbNode`] records.
///
/// Nodes are linked via `parent_id`. Nodes without a `parent_id` (or whose
/// `parent_id` does not match any node in the list) are treated as roots.
///
/// # Arguments
///
/// * `nodes` - A flat slice of [`DbNode`] records, typically from a database query.
///
/// # Returns
///
/// A `Vec<FsNode>` containing the root nodes of the reconstructed hierarchy.
/// Each root's `children` field is recursively populated.
///
/// # Examples
///
///