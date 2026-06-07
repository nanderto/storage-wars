//! Flatten a nested `FsNode` tree into a `Vec<UiNode>` respecting expanded paths,
//! computing `scan_progress` as a fraction of the largest sibling.