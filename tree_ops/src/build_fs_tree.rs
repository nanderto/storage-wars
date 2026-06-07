//! Reconstructs an [`FsNode`] hierarchy from a flat [`DbNode`] list via `parent_id`.

use std::collections::HashMap;
use crate::models::{DbNode, FsNode};

/// Reconstructs a forest of [`FsNode`] trees from a flat list of [`DbNode`] records.
///
/// Nodes whose `parent_id` is `None` become root nodes. All other nodes are
/// attached to their parent. Nodes referencing a non-existent `parent_id` are
/// silently promoted to root nodes.
///
/// # Arguments
///
/// * `nodes` — Flat list of database node records.
///
/// # Returns
///
/// A `Vec<FsNode>` containing the root nodes of the reconstructed forest.
///
/// # Examples
///
///