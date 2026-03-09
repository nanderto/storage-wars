# Storage Wars - Disk Storage Monitor (Rust GPUI)

## Context

Brand-new desktop application to track disk folder growth over time. Users select one drive from a list of all available drives, optionally view scan history, and click "Scan Now" to run a new scan. Results are displayed in a collapsible hierarchical tree with color-coded size-change status bars. SQLite stores full scan history, allowing any two scans to be compared.

## Confirmed Design Decisions

- **Drive selection**: List all available drives; user picks one
- **Scan trigger**: Manual "Scan Now" button (user can review old scans without rescanning)
- **History**: Full scan history stored; user can pick any two scans to compare
- **Tree default**: Starts fully collapsed (top-level folders only)
- **Storage**: SQLite via `rusqlite` (bundled, no system dependency)

---

## Cargo.toml

```toml
[package]
name = "storage-wars"
version = "0.1.0"
edition = "2021"

[dependencies]
gpui = { git = "https://github.com/zed-industries/zed", branch = "main" }
gpui-component = { git = "https://github.com/longbridge/gpui-component", branch = "main" }
serde = { version = "1", features = ["derive"] }
anyhow = "1"
dirs = "5"
sysinfo = "0.30"
rusqlite = { version = "0.31", features = ["bundled"] }

[dev-dependencies]
tempfile = "3"
```

---

## Module Structure

```
src/
  main.rs           - App entry point, window creation
  app_view.rs       - Root view: drive selector + scan history panel + tree area
  drive_selector.rs - Drive list panel (all available drives, pick one)
  scan_history.rs   - Scan history list + "pick two to compare" UI
  tree_view.rs      - Hierarchical file/folder tree with status bars
  scanner.rs        - Async recursive directory scanner
  persistence.rs    - SQLite open/migrate/save/query
  models.rs         - All shared data types
```

---

## Key Data Structures (`models.rs`)

```rust
#[derive(Clone)]
pub struct FsNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub current_size: u64,
    pub prev_size: Option<u64>,  // populated when comparing two scans
    pub children: Vec<FsNode>,
}

pub struct UiNode {
    pub fs_node: FsNode,
    pub depth: usize,
    pub expanded: bool,
    pub scan_progress: f32,      // 0.0..=1.0
}

pub enum SizeChange {
    NoBaseline,
    Decreased,
    Unchanged,
    SmallGrowth,    // 0–50% increase
    MediumGrowth,   // 50–100% increase
    LargeGrowth,    // 100%+ increase
}

pub struct ScanMeta {
    pub id: i64,
    pub drive: String,
    pub scanned_at: String,      // ISO 8601 timestamp
    pub total_size: u64,
}

pub struct DbNode {
    pub id: i64,
    pub scan_id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
}
```

---

## SQLite Schema (`persistence.rs`)

Database location: `{APPDATA}/storage-wars/storage-wars.db`

```sql
CREATE TABLE IF NOT EXISTS scans (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  drive       TEXT    NOT NULL,
  scanned_at  TEXT    NOT NULL,
  total_size  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS nodes (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  scan_id     INTEGER NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
  parent_id   INTEGER,
  name        TEXT    NOT NULL,
  path        TEXT    NOT NULL,
  is_dir      BOOLEAN NOT NULL,
  size        INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_nodes_scan  ON nodes(scan_id);
CREATE INDEX IF NOT EXISTS idx_nodes_path  ON nodes(path);
```

Key functions:
- `open_db() -> Connection` — create/migrate schema
- `save_scan(conn, drive, root_node) -> i64` — flatten tree, bulk insert in one transaction
- `get_scans_for_drive(conn, drive) -> Vec<ScanMeta>`
- `load_scan_tree(conn, scan_id) -> Vec<DbNode>` — fetch all nodes, caller rebuilds tree
- `delete_scan(conn, scan_id)` — cascade deletes nodes

---

## App State (`app_view.rs`)

```rust
pub struct AppState {
    pub drives: Vec<DriveInfo>,
    pub selected_drive: Option<String>,
    pub scan_history: Vec<ScanMeta>,
    pub compare_a: Option<i64>,          // scan_id for left/base comparison
    pub compare_b: Option<i64>,          // scan_id for right/new comparison
    pub flat_nodes: Vec<UiNode>,         // current visible flattened tree
    pub scanning: bool,
    pub scan_progress: f32,
}
```

---

## Scanner Flow (`scanner.rs`)

1. User clicks "Scan Now" → `start_scan(drive, state, cx)`
2. `cx.background_spawn(scan_dir(root))` — depth-first recursive walk
   - Recurse into subdirs first → sum children sizes bottom-up
   - Batch progress updates every 500 nodes via `cx.spawn` → `cx.notify()`
3. On completion (foreground `cx.spawn`):
   - Call `persistence::save_scan(conn, drive, root_node)`
   - Refresh `state.scan_history`
   - If a compare_a is selected, merge prev_size from that scan
   - Rebuild `flat_nodes` (collapsed by default)
   - `cx.notify()`

---

## Tree View Rendering (`tree_view.rs`)

Row layout:
```
[──indent──][▶/▼] [📁/📄] Name            1.2 GB  │▓▓▓▓▓░░░░░░░░░░│
                                                    └─ colored bar ──┘
```

- Left: `flex_row`, indent = `depth * 16px`, chevron toggle, icon, name, size
- Right: `w_full` relative container:
  - Background bar (grey, full width)
  - Colored overlay bar width = `scan_progress * 100%`
  - Color from `SizeChange`:
    - `Decreased`    → `#22c55e` (green)
    - `SmallGrowth`  → `#eab308` (yellow)
    - `MediumGrowth` → `#f97316` (orange)
    - `LargeGrowth`  → `#ef4444` (red)
    - `NoBaseline`   → `#6b7280` (grey)

Use **VirtualList** (gpui-component) — only renders visible rows.

Expand/collapse: chevron click → toggle `UiNode.expanded` → re-flatten → `cx.notify()`.

---

## Scan History Panel (`scan_history.rs`)

- List of past scans for selected drive (date + total size)
- Two selection modes: "Base" (compare_a) and "New" (compare_b)
- "Compare" button loads both scans, merges `prev_size`, rebuilds tree
- "Delete" button removes a scan from DB

---

## Testing Strategy

Unit tests live as `#[cfg(test)]` modules **within the same file** as the code they test. UI modules (`drive_selector`, `scan_history`, `tree_view`, `app_view`, `main`) are excluded — GPUI has no headless test harness and these are covered by manual verification.

### `models.rs` — Pure logic tests
- `SizeChange::from_node()`: boundary cases — decreased, unchanged, all three growth bands, zero prev_size
- `format_size()`: bytes, KB, MB, GB, TB formatting

### `persistence.rs` — SQLite integration tests (in-memory DB)
Use `rusqlite::Connection::open_in_memory()` so tests never touch the filesystem.
- Schema migration runs without error
- `save_scan()` + `load_scan_tree()` roundtrip: tree is saved and all nodes retrieved
- `get_scans_for_drive()`: filters correctly by drive name
- `delete_scan()`: cascade-deletes associated nodes

### `scanner.rs` — Logic + filesystem tests
- `flatten_tree()`: collapsed tree returns only root nodes; expanding a node surfaces its children at depth+1
- `merge_baseline()`: correctly populates `prev_size` by path match; unmatched paths stay `None`
- `build_baseline_map()`: produces correct path→size map from `DbNode` slice
- `scan_dir_sync()`: uses `tempfile::TempDir` to build a small known directory tree, verifies sizes are summed correctly bottom-up

### Running tests
```bash
cargo test                        # all tests
cargo test -- --nocapture         # with stdout
cargo test persistence            # filter by module name
```

---

## File Creation Order (Tasks)

### Phase 0 — Project Initialization ✅
1. ✅ `cargo init` in `C:\dev\Github\repos\storage-wars`
2. ✅ Create `.gitignore`
3. ✅ `git init`
4. ✅ Initial commit + push to GitHub
5. ✅ Write `Tasks.md` to project root
6. ✅ Write `CLAUDE.md` to project root
7. Write full `PLAN.md` to project root (this document — complete implementation spec)
8. Update `Tasks.md` to include all testing checklist items

### Phase 1 — Data Layer
9. Create branch `phase-1-data-layer`
10. Edit `Cargo.toml` — add all dependencies + `[dev-dependencies] tempfile = "3"`
11. Create `src/models.rs`
12. Add `#[cfg(test)]` to `models.rs`: `SizeChange::from_node()` boundary tests + `format_size()` tests
13. Create `src/persistence.rs`
14. Add `#[cfg(test)]` to `persistence.rs`: schema migration, save/load roundtrip, filter by drive, delete cascade (all using in-memory SQLite)
15. `cargo test` — all tests pass
16. Commit, push, open PR

### Phase 2 — Scanner
17. Create branch `phase-2-scanner`
18. Create `src/scanner.rs`
19. Add `#[cfg(test)]` to `scanner.rs`: `flatten_tree()` collapse/expand, `merge_baseline()`, `build_baseline_map()`, `scan_dir_sync()` with `tempfile::TempDir`
20. `cargo test` — all tests pass
21. Commit, push, open PR

### Phase 3 — UI Components
22. Create branch `phase-3-ui-components`
23. Create `src/drive_selector.rs`
24. Create `src/scan_history.rs`
25. Create `src/tree_view.rs`
26. `cargo check` — no errors
27. Commit, push, open PR

### Phase 4 — App Wiring
28. Create branch `phase-4-app-wiring`
29. Create `src/app_view.rs`
30. Edit `src/main.rs` — full entry point
31. `cargo check` — no errors
32. Commit, push, open PR

### Phase 5 — Build & Fix
33. Create branch `phase-5-build-fix`
34. `cargo build` — resolve all compile errors
35. `cargo test` — all tests pass
36. Commit: `"feat: initial storage-wars implementation"`, push, open PR

---

## Verification

- `cargo build` — no errors
- `cargo test` — all unit/integration tests pass
- App opens → shows all drives (C:\, D:\, etc.)
- Select a drive → scan history panel shows (empty on first run)
- Click "Scan Now" → tree populates live, progress bars animate
- Reopen app → scan history shows previous scans
- Select two scans + "Compare" → tree shows color-coded deltas
- Large drive: VirtualList renders smoothly at 60fps
