//! Finds a parent node by path and replaces its children.

use std::path::PathBuf;
use crate::models::FsNode;

/// Finds the node at `parent_path` within the tree rooted at `root` and
/// replaces its `children` with `new_children`.
///
/// The search is performed depth-first. Returns `true` if the parent was
/// found and updated, `false` otherwise.
///
/// # Arguments
///
/// * `root` - Mutable reference to the root of the tree to search.
/// * `parent_path` - The path of the node whose children should be replaced.
/// * `new_children` - The replacement children to assign.
///
/// # Examples
///
///