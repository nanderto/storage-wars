//! # tree_ops
//!
//! Tree manipulation utilities for FsNode hierarchy operations.
//!
//! ## Overview
//!
//! This crate provides utilities for:
//! - [`build_fs_tree`]: Reconstructs `FsNode` hierarchy from a flat `DbNode` list via `parent_id`.
//! - [`flatten_tree`]: Converts nested `FsNode` to `Vec<UiNode>` respecting `expanded_paths`
//!   with `scan_progress` as a fraction of the largest sibling.
//! - [`recalculate_sizes`]: Walks bottom-up summing sizes and counts.
//! - [`insert_children`]: Finds a parent node and replaces its children.
//! - [`build_baseline_map`]: Creates a `PathBuf → u64` lookup map.
//! - [`merge_baseline`]: Populates `prev_size` from a baseline map.

pub mod baseline;
pub mod flatten;
pub mod insert;
pub mod recalculate;
pub mod tree_builder;
pub mod types;

pub use baseline::{build_baseline_map, merge_baseline};
pub use flatten::flatten_tree;
pub use insert::insert_children;
pub use recalculate::recalculate_sizes;
pub use tree_builder::build_fs_tree;
pub use types::{DbNode, FsNode, UiNode};