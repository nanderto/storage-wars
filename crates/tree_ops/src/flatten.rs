//! Flatten a nested `FsNode` tree into a `Vec<UiNode>` respecting
//! `expanded_paths`, with `scan_progress` expressed as a fraction of
//! the largest sibling.

// TODO: implement flatten_tree