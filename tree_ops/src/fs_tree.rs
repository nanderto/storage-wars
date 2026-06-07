//! Reconstructs an [`FsNode`] hierarchy from a flat [`DbNode`] list.

use std::collections::HashMap;

use crate::types::{DbNode, FsNode};

/// Reconstructs a forest of [`FsNode`] trees from a flat list of [`DbNode`] records.
///
/// Nodes are linked via `parent_id`. Nodes whose `parent_id` is `None` or references
/// an id not present in the list become root nodes.
///
/// # Arguments
///
/// * `db_nodes` — A flat list of database node records.
///
/// # Returns
///
/// A `Vec<FsNode>` containing the root nodes of the reconstructed tree(s).
///
/// # Examples
///
///