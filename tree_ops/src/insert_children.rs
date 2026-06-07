//! Finds a parent [`FsNode`] by path and replaces its children.

use std::path::Path;
use crate::models::FsNode;

/// Searches the tree rooted at `roots` for a node whose `path` equals
/// `parent_path`, then replaces that node's `children` with `new_children`.
///
/// The search is depth-first. Returns `true` if the parent was found and
/// updated, or `false` if no matching node was found.
///
/// # Arguments
///
/// * `roots`       — Mutable slice of root nodes to search.
/// * `parent_path` — Path of the node whose children should be replaced.
/// * `new_children`— Replacement children to assign.
///
/// # Examples
///
///