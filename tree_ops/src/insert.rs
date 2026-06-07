//! Inserts or replaces children of a target node identified by path.

use std::path::PathBuf;

use crate::types::FsNode;

/// Finds the node at `target_path` in the tree and replaces its children
/// with `new_children`.
///
/// The search is performed depth-first. Returns `true` if the target node
/// was found and updated, or `false` if no node with `target_path` exists.
///
/// # Arguments
///
/// * `roots` - A mutable slice of root `FsNode` entries to search.
/// * `target_path` - The path of the node whose children should be replaced.
/// * `new_children` - The replacement children to insert.
///
/// # Examples
///
///