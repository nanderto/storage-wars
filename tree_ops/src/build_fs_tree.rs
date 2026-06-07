//! Reconstructs an `FsNode` hierarchy from a flat list of `DbNode` records.

use std::collections::HashMap;
use crate::models::{DbNode, FsNode};

/// Builds a forest of `FsNode` trees from a flat list of `DbNode` records.
///
/// Nodes are linked via `parent_id`. Nodes whose `parent_id` is `None` or whose
/// parent is not present in the list become root nodes.
///
/// # Arguments
///
/// * `nodes` - A flat list of database node records.
///
/// # Returns
///
/// A `Vec<FsNode>` containing the root nodes of the reconstructed hierarchy,
/// with children populated recursively.
///
/// # Examples
///
///