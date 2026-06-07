//! Rebuild an FsNode hierarchy from a flat list of DbNodes using parent_id
//! relationships (build_fs_tree), and related tree-manipulation utilities:
//! recalculate_sizes, insert_children, build_baseline_map, merge_baseline.

// TODO: implement rebuild functions