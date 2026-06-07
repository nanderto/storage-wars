//! Reconstructs an `FsNode` hierarchy from a flat `DbNode` list via `parent_id`.

use std::collections::HashMap;
use crate::models::{DbNode, FsNode};

/// Reconstructs a forest of `FsNode` trees from a flat list of `DbNode` records.
///
/// Nodes are linked via `parent_id`. Nodes whose `parent_id` is `None` or whose
/// parent is not present in the list become root nodes.
///
/// # Arguments
///
/// * `nodes` - A flat slice of `DbNode` records (order is not significant).
///
/// # Returns
///
/// A `Vec<FsNode>` containing the root nodes of the reconstructed forest.
/// Each root's `children` are recursively populated.
///
/// # Examples
///
///