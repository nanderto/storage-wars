# Storage Wars — Task Checklist

## Phase 0 — Project Initialization
- [x] `cargo init` in `C:\dev\Github\repos\storage-wars`
- [x] Create `.gitignore` (standard Rust: `/target`, `Cargo.lock`)
- [x] `git init`
- [x] Initial commit + push to GitHub
- [x] Write `Tasks.md` to project root
- [x] Write `CLAUDE.md` to project root
- [ ] Write full `PLAN.md` to project root
- [ ] Update `Tasks.md` to include all testing checklist items
- [ ] Commit Phase 0 changes, push

## Phase 1 — Data Layer
- [ ] Create branch `phase-1-data-layer`
- [ ] Edit `Cargo.toml` — add all dependencies + `[dev-dependencies] tempfile = "3"`
- [ ] Create `src/models.rs`
- [ ] `models.rs` tests: `SizeChange::from_node()` boundary cases (decreased, unchanged, small/medium/large growth, zero prev_size)
- [ ] `models.rs` tests: `format_size()` (bytes, KB, MB, GB, TB)
- [ ] Create `src/persistence.rs`
- [ ] `persistence.rs` tests: schema migration runs without error (in-memory DB)
- [ ] `persistence.rs` tests: `save_scan()` + `load_scan_tree()` roundtrip
- [ ] `persistence.rs` tests: `get_scans_for_drive()` filters by drive name
- [ ] `persistence.rs` tests: `delete_scan()` cascade-deletes associated nodes
- [ ] `cargo test` — all tests pass
- [ ] Commit, push, open PR

## Phase 2 — Scanner
- [ ] Create branch `phase-2-scanner`
- [ ] Create `src/scanner.rs`
- [ ] `scanner.rs` tests: `flatten_tree()` collapsed returns only root nodes
- [ ] `scanner.rs` tests: `flatten_tree()` expanding a node surfaces children at depth+1
- [ ] `scanner.rs` tests: `merge_baseline()` populates `prev_size` by path match; unmatched paths stay `None`
- [ ] `scanner.rs` tests: `build_baseline_map()` produces correct path→size map from `DbNode` slice
- [ ] `scanner.rs` tests: `scan_dir_sync()` with `tempfile::TempDir` — sizes summed correctly bottom-up
- [ ] `cargo test` — all tests pass
- [ ] Commit, push, open PR

## Phase 3 — UI Components
- [ ] Create branch `phase-3-ui-components`
- [ ] Create `src/drive_selector.rs`
- [ ] Create `src/scan_history.rs`
- [ ] Create `src/tree_view.rs`
- [ ] `cargo check` — no errors
- [ ] Commit, push, open PR

## Phase 4 — App Wiring
- [ ] Create branch `phase-4-app-wiring`
- [ ] Create `src/app_view.rs`
- [ ] Edit `src/main.rs` — full entry point
- [ ] `cargo check` — no errors
- [ ] Commit, push, open PR

## Phase 5 — Build & Fix
- [ ] Create branch `phase-5-build-fix`
- [ ] `cargo build` — resolve all compile errors
- [ ] `cargo test` — all tests pass
- [ ] Commit: `"feat: initial storage-wars implementation"`, push, open PR
