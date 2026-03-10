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
- [x] Commit, push, open PR

## Phase 5 — Build & Fix
- [x] Create branch `phase-5-build-fix`
- [x] `cargo build` — resolve all compile errors
- [x] `cargo test` — all tests pass (49 tests, 0 warnings)
- [x] `cargo clippy -- -D warnings` — clean
- [x] Commit, push, open PR

## Phase 6 — UI Restructure (Custom Chrome + WizTree-style Explorer)

### 6a. Dependencies + Models
- [x] Add `gpui-component` to `Cargo.toml` (pinned to compatible rev)
- [x] `models.rs`: Add `volume_label: String` to `DriveInfo`
- [x] `models.rs`: Add `file_count: u64`, `folder_count: u64`, `modified: Option<String>` to `FsNode`
- [x] `models.rs`: Add `file_count`, `folder_count`, `modified` to `DbNode`
- [x] Add `format_number()` helper (comma-formatted counts, e.g. "2,979,238")
- [x] Update all existing tests constructing FsNode/DbNode/DriveInfo with new fields
- [x] `cargo test` — all 52 tests pass
- [x] `cargo clippy -- -D warnings` — clean

### 6b. Data Layer Updates
- [x] `persistence.rs`: Add `file_count`, `folder_count`, `modified` columns to `nodes` table schema
- [x] `persistence.rs`: Update `save_scan()` INSERT to include new fields
- [x] `persistence.rs`: Update `load_scan_tree()` SELECT to read new fields
- [x] `scanner.rs`: Collect recursive file/folder counts during scan
- [x] `scanner.rs`: Collect `modified` timestamp from `fs::metadata().modified()`
- [x] Update persistence tests (save/load roundtrip includes new fields)
- [x] Update scanner tests (scan_dir_sync verifies counts + modified)
- [x] `cargo test` — all 55 tests pass

### 6c. Window Chrome + Root Wrapper
- [x] `main.rs`: Call `gpui_component::init(cx)` before opening window
- [x] `main.rs`: Use `TitleBar::title_bar_options()` for `WindowOptions.titlebar`
- [x] `main.rs`: Wrap `AppView` in `Root::new()` for theming/popups
- [x] `main.rs`: Enrich `enumerate_drives()` with `volume_label` from `sysinfo`
- [x] `cargo check` — compiles with new gpui-component dependency
- [x] `app_view.rs`: Render TitleBar at top, restructure layout (title → toolbar → content)

### 6d. Drive Selector Rewrite
- [x] `drive_selector.rs`: Replace sidebar layout with gpui-component `Select` dropdown
- [x] Implement `DriveSelectItem` struct with `SelectItem` trait
- [x] Subscribe to `SelectEvent::Confirm` → emit `DriveSelectorEvent::DriveSelected`
- [x] Update drive_selector tests for new Select-based internals
- [x] `cargo test` — drive_selector tests pass

### 6e. Tree View — Columnar Explorer
- [x] `tree_view.rs`: Add column header row (Name, % of Parent, Size, Prev Size, % Prev, Files, Folders, Modified)
- [x] Render each data row with fixed-width columns (right-aligned numbers)
- [x] Name column: tree indent + chevron + icon + name (flex-grow)
- [x] % of Parent column: `scan_progress * 100` as "XX.X %" text
- [x] Size / Prev Size columns: `format_size()` values
- [x] % of Previous Size: computed change percentage
- [x] Files / Folders columns: `format_number()` counts
- [x] Modified column: timestamp string
- [x] Update tree_view tests
- [x] `cargo test` — tree_view tests pass

### 6f. App Layout Restructure
- [x] `app_view.rs`: Add `drives`, `scan_item_count`, `last_scan_time` fields
- [x] `app_view.rs`: Change constructor to accept `window: &mut Window` param
- [x] Render: Row 1 — `TitleBar::new().child("Storage Wars")`
- [x] Render: Row 2 — Toolbar (DriveSelector dropdown + Scan button + usage bar + drive properties)
- [x] Render: Row 3 — Main content (ScanHistory 280px sidebar + TreeView flex-grow)
- [x] Render: Row 4 — Status bar (item count + drive path + last scan time)
- [x] Update `set_drives()` to store drives locally for toolbar display
- [x] Update scan/compare handlers to set `scan_item_count` and `last_scan_time`
- [x] `cargo check` — compiles

### 6g. Final Verification
- [x] `cargo clippy -- -D warnings` — clean
- [x] `cargo test` — all tests pass
- [ ] `cargo run` — manual verification
- [x] Commit, push, open PR

## Phase 7 — Incremental Parallel Scanner

### 7a. Dependencies
- [x] Add `async-channel = "2"` to `[dependencies]` in `Cargo.toml`

### 7b. Scanner Module (`scanner.rs`)
- [x] Add `ScanMessage` enum (`DirScanned`, `ScanError`, `Complete`)
- [x] Add `read_dir_immediate(dir)` — reads one folder without recursion
- [x] Add `scan_dir_incremental(root, tx, cancelled, num_workers)` — parallel scanner with work queue + condvar
- [x] Add `insert_children(root, parent_path, children)` — insert into tree at path
- [x] Add `recalculate_sizes(node)` — bottom-up size/count recalculation
- [x] Tests: `read_dir_immediate` (files+dirs, empty, nonexistent)
- [x] Tests: `insert_children` (at root, nested, missing parent)
- [x] Tests: `recalculate_sizes` (bottom-up propagation)
- [x] Tests: `scan_dir_incremental` (full scan, cancellation)

### 7c. App Wiring (`app_view.rs`)
- [x] Add `scan_cancel: Arc<AtomicBool>` and `dirs_scanned: usize` fields
- [x] Rewrite `start_scan()` — channel-based async loop with incremental tree updates
- [x] Cancel support — clicking "Scan" during scan sets cancel flag, partial tree stays visible
- [x] "Scan" button shows "Cancel" (red) while scanning
- [x] Status text shows "Scanning… (N dirs)" during scan

### 7d. Verification
- [x] `cargo check` — compiles
- [x] `cargo clippy -- -D warnings` — clean
- [x] `cargo test` — all 68 tests pass (12 new: 9 scanner + 3 app_view)
- [ ] `cargo run` — manual verification
- [ ] Commit, push, open PR
