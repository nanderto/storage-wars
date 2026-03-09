# Storage Wars — Task Checklist

## Phase 0 — Project Initialization
- [x] `cargo init` in `C:\dev\Github\repos\storage-wars`
- [x] Create `.gitignore` (standard Rust: `/target`, `Cargo.lock`)
- [x] `git init`
- [x] Initial commit + push to GitHub
- [x] Write `Tasks.md` to project root
- [x] Write `CLAUDE.md` to project root
- [x] Write full `PLAN.md` to project root
- [x] Update `Tasks.md` to include all testing checklist items
- [x] Commit Phase 0 changes, push

## Phase 1 — Data Layer
- [x] Create branch `phase-1-data-layer`
- [x] Edit `Cargo.toml` — add all dependencies + `[dev-dependencies] tempfile = "3"`
- [x] Create `src/models.rs`
- [x] `models.rs` tests: `SizeChange::from_node()` boundary cases (decreased, unchanged, small/medium/large growth, zero prev_size)
- [x] `models.rs` tests: `format_size()` (bytes, KB, MB, GB, TB)
- [x] Create `src/persistence.rs`
- [x] `persistence.rs` tests: schema migration runs without error (in-memory DB)
- [x] `persistence.rs` tests: `save_scan()` + `load_scan_tree()` roundtrip
- [x] `persistence.rs` tests: `get_scans_for_drive()` filters by drive name
- [x] `persistence.rs` tests: `delete_scan()` cascade-deletes associated nodes
- [x] `cargo test` — all tests pass
- [x] Commit, push, open PR

## Phase 2 — Scanner
- [x] Create branch `phase-2-scanner`
- [x] Create `src/scanner.rs`
- [x] `scanner.rs` tests: `flatten_tree()` collapsed returns only root nodes
- [x] `scanner.rs` tests: `flatten_tree()` expanding a node surfaces children at depth+1
- [x] `scanner.rs` tests: `merge_baseline()` populates `prev_size` by path match; unmatched paths stay `None`
- [x] `scanner.rs` tests: `build_baseline_map()` produces correct path→size map from `DbNode` slice
- [x] `scanner.rs` tests: `scan_dir_sync()` with `tempfile::TempDir` — sizes summed correctly bottom-up
- [x] `cargo test` — all tests pass
- [x] Commit, push, open PR

## Phase 3 — UI Components
- [x] Create branch `phase-3-ui-components`
- [x] Create `src/drive_selector.rs`
- [x] `drive_selector.rs` tests: initial state, set_drives, selected_drive mutation, event emission, render smoke
- [x] Create `src/scan_history.rs`
- [x] `scan_history.rs` tests: initial state, set_scans, compare_a/b selection, CompareRequested/DeleteRequested emission, render smoke
- [x] Create `src/tree_view.rs`
- [x] `tree_view.rs` tests: initial state, set_nodes, ToggleExpand emission, render smoke
- [x] Enable `gpui test-support` feature in `[dev-dependencies]`
- [x] `cargo test` — all 49 tests pass
- [x] `cargo clippy -- -D warnings` — clean
- [x] Commit, push, open PR

## Phase 4 — App Wiring
- [x] Create branch `phase-4-app-wiring`
- [x] Create `src/app_view.rs`
- [x] Edit `src/main.rs` — full entry point
- [x] `cargo check` — no errors
- [ ] Commit, push, open PR

## Phase 5 — Build & Fix
- [ ] Create branch `phase-5-build-fix`
- [ ] `cargo build` — resolve all compile errors
- [ ] `cargo test` — all tests pass
- [ ] Commit: `"feat: initial storage-wars implementation"`, push, open PR
