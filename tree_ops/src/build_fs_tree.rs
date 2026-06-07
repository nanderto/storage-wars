//! Reconstructs an `FsNode` hierarchy from a flat list of `DbNode` entries.

use std::collections::HashMap;
use crate::models::{DbNode, FsNode};

/// Builds a forest of `FsNode` trees from a flat slice of `DbNode` entries.
///
/// Nodes are linked via `parent_id`. Nodes whose `parent_id` is `None` or whose
/// parent is not present in the list become roots of the returned forest.
///
/// # Arguments
///
/// * `nodes` — A flat list of database nodes (order does not matter).
///
/// # Returns
///
/// A `Vec<FsNode>` containing the root nodes of the reconstructed hierarchy.
///
/// # Examples
///
///