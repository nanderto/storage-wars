//! # tree_ops
//!
//! Tree manipulation utilities for filesystem node hierarchies.
//!
//! ## Overview
//!
//! This crate provides utilities for working with filesystem node trees:
//!
//! - [`build_fs_tree`] — Reconstructs an [`FsNode`] hierarchy from a flat [`DbNode`] list via `parent_id`.
//! - [`flatten_tree`] — Converts a nested [`FsNode`] to `Vec<UiNode>` respecting `expanded_paths`
//!   with `scan_progress` as a fraction of the largest sibling.
//! - [`recalculate_sizes`] — Walks bottom-up summing sizes and counts.
//! - [`insert_children`] — Finds a parent node and replaces its children.
//! - [`build_baseline_map`] — Creates a `PathBuf → u64` lookup map.
//! - [`merge_baseline`] — Populates `prev_size` from a baseline map.

pub mod baseline;
pub mod flatten;
pub mod fs_tree;
pub mod insert;
pub mod sizes;
pub mod types;

pub use baseline::{build_baseline_map, merge_baseline};
pub use flatten::flatten_tree;
pub use fs_tree::build_fs_tree;
pub use insert::insert_children;
pub use sizes::recalculate_sizes;
pub use types::{DbNode, FsNode, UiNode};