//! Tree manipulation utilities for filesystem node hierarchies.
//!
//! Provides operations such as rebuilding an `FsNode` tree from flat `DbNode`
//! records, flattening a tree into `UiNode` lists, recalculating sizes,
//! inserting children, and baseline map operations.

/// Flatten a nested `FsNode` tree into a `Vec<UiNode>` respecting expanded paths,
/// with `scan_progress` computed as a fraction of the largest sibling.
pub mod flatten {
    // TODO: implement flatten_tree
}

/// Rebuild an `FsNode` hierarchy from a flat `Vec<DbNode>` using `parent_id` links.
pub mod rebuild {
    // TODO: implement build_fs_tree
}