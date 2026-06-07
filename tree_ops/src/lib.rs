//! # tree_ops
//!
//! Tree manipulation utilities for filesystem node hierarchies.
//!
//! ## Overview
//!
//! This crate provides the following core operations:
//!
//! - [`build_fs_tree`]: Reconstructs an `FsNode` hierarchy from a flat `DbNode` list via `parent_id`.
//! - [`flatten_tree`]: Converts a nested `FsNode` to `Vec<UiNode>` respecting `expanded_paths`
//!   with `scan_progress` as a fraction of the largest sibling.
//! - [`recalculate_sizes`]: Walks the tree bottom-up, summing sizes and counts.
//! - [`insert_children`]: Finds a parent node and replaces its children.
//! - [`build_baseline_map`]: Creates a `PathBuf → u64` lookup map.
//! - [`merge_baseline`]: Populates `prev_size` from a baseline map.

pub mod models;
pub mod build_fs_tree;
pub mod flatten_tree;
pub mod recalculate_sizes;
pub mod insert_children;
pub mod baseline;

pub use models::{DbNode, FsNode, UiNode};
pub use build_fs_tree::build_fs_tree;
pub use flatten_tree::flatten_tree;
pub use recalculate_sizes::recalculate_sizes;
pub use insert_children::insert_children;
pub use baseline::{build_baseline_map, merge_baseline};