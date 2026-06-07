//! Flatten a nested `FsNode` tree into a `Vec<UiNode>` respecting expanded paths,
//! with `scan_progress` computed as a fraction of the largest sibling.