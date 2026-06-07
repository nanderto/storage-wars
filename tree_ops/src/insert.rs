//! Inserts or replaces children of a target node identified by path.

use std::path::PathBuf;

use crate::types::FsNode;

/// Finds the node at `target_path` in the tree and replaces its children with `new_children`.
///
/// Performs a depth-first search. Returns `true` if the target node was found and
/// updated, `false` otherwise.
///
/// # Arguments
///
/// * `nodes` — A mutable slice of root [`FsNode`]s to search.
/// * `target_path` — The path of the node whose children should be replaced.
/// * `new_children` — The replacement children to insert.
///
/// # Examples
///
///