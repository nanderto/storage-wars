# Storage Wars - Disk Storage Monitor (Rust GPUI)

## Context

Desktop application (Windows-first) to track disk folder growth over time. Users select a drive from a dropdown, click "Scan Now" to run a recursive scan, and view results in a WizTree-style columnar explorer. SQLite stores full scan history, allowing any two scans to be compared with color-coded size deltas.

## Status

- **Phases 0–6g**: Complete (see `Tasks.md` for detailed checklist)
- **Current branch**: `phase-6-ui-restructure` (PR #7 open)
- **Tests**: 56 passing, clippy clean
- **Next step**: `cargo run` manual verification, then merge PR #7

## Confirmed Design Decisions

- **Drive selection**: Dropdown (gpui-component `Select` widget); user picks one
- **Scan trigger**: Manual "Scan Now" button — selecting a drive does NOT auto-scan
- **Drive root on select**: Immediately shows drive as a single unexpanded node; loads most recent scan from DB if one exists
- **History**: Full scan history stored in SQLite; any two scans can be compared
- **Tree default**: Starts fully collapsed (top-level folders only)
- **Storage**: SQLite via `rusqlite` (bundled, no system dependency)
- **Theme**: Dark (Catppuccin Mocha palette), custom title bar with OS-native window controls
- **No scan history sidebar**: Removed in Phase 6f — tree view fills the entire content area

---

## Dependencies (`Cargo.toml`)

```toml
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed" }
gpui_platform = { git = "https://github.com/zed-industries/zed" }
gpui-component = { git = "https://github.com/longbridge/gpui-component", rev = "b6f284ce" }
serde = { version = "1", features = ["derive"] }
anyhow = "1"
dirs = "5"
sysinfo = "0.30"
rusqlite = { version = "0.31", features = ["bundled"] }

[dev-dependencies]
tempfile = "3"
gpui = { git = "https://github.com/zed-industries/zed", features = ["test-support"] }
```

---

## Module Structure

```
src/
  main.rs           - Entry point: enumerate drives, open window, init theme
  lib.rs            - Declares all pub modules
  app_view.rs       - Root view: custom title bar, toolbar, tree area, status bar
  drive_selector.rs - Drive dropdown (gpui-component Select widget)
  scan_history.rs   - Scan history list + compare UI (wired but sidebar hidden)
  tree_view.rs      - WizTree-style columnar explorer with 9 columns
  scanner.rs        - Synchronous recursive directory scanner (background thread)
  persistence.rs    - SQLite open/migrate/save/query
  models.rs         - All shared data types + formatting helpers
```

---

## Key Data Structures (`models.rs`)

```rust
pub struct FsNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub current_size: u64,
    pub prev_size: Option<u64>,     // populated when comparing two scans
    pub children: Vec<FsNode>,
    pub file_count: u64,            // recursive file count
    pub folder_count: u64,          // recursive folder count
    pub modified: Option<String>,   // ISO 8601 timestamp
}

pub struct UiNode {
    pub fs_node: FsNode,
    pub depth: usize,
    pub expanded: bool,
    pub scan_progress: f32,         // 0.0..=1.0 (fraction of parent)
}

pub enum SizeChange { NoBaseline, Decreased, Unchanged, SmallGrowth, MediumGrowth, LargeGrowth }

pub struct ScanMeta { pub id: i64, pub drive: String, pub scanned_at: String, pub total_size: u64 }

pub struct DbNode {
    pub id: i64, pub scan_id: i64, pub parent_id: Option<i64>,
    pub name: String, pub path: String, pub is_dir: bool, pub size: u64,
    pub file_count: u64, pub folder_count: u64, pub modified: Option<String>,
}

pub struct DriveInfo {
    pub name: String,             // e.g. "C:"
    pub volume_label: String,     // e.g. "OS"
    pub total_space: u64,
    pub available_space: u64,
}
```

Helpers: `format_size(bytes) -> String`, `format_number(n) -> String` (comma-separated)

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
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  scan_id      INTEGER NOT NULL REFERENCES scans(id) ON DELETE CASCADE,
  parent_id    INTEGER,
  name         TEXT    NOT NULL,
  path         TEXT    NOT NULL,
  is_dir       BOOLEAN NOT NULL,
  size         INTEGER NOT NULL,
  file_count   INTEGER NOT NULL DEFAULT 0,
  folder_count INTEGER NOT NULL DEFAULT 0,
  modified     TEXT
);
```

Key functions: `open_db()`, `save_scan()`, `get_scans_for_drive()`, `load_scan_tree()`, `delete_scan()`

---

## App Layout (`app_view.rs`)

```
┌─────────────────────────────────────────────────────────────┐
│ Row 1: Custom Title Bar                                     │
│   [Storage Wars]          [⚙] [—] [◻] [✕]                  │
│   (drag area)             (settings, min, max, close)       │
├─────────────────────────────────────────────────────────────┤
│ Row 2: Toolbar                                              │
│   Select: [▼ Drive dropdown] [Scan]    Selection: [C:] OS   │
│   Ready  ████████████████████           Total Space: 500 GB  │
│                                         Space Used:  400 GB  │
│                                         Space Free:  100 GB  │
├─────────────────────────────────────────────────────────────┤
│ Row 3: Tree View (full width, scrollable)                   │
│   Name          % Parent  ████  Size  Prev  % Prev  Files…  │
│   ▶ 💾 C:\      100.0%    ████  1 TB  —     —       42     │
│     ▶ 📁 Users   45.2%    ██    450G  —     —       20     │
│     ▶ 📁 Windows 30.1%    ██    300G  —     —       15     │
├─────────────────────────────────────────────────────────────┤
│ Row 4: Status Bar                                           │
│   1,234 items              C:\           Last scan: 2026-…   │
└─────────────────────────────────────────────────────────────┘
```

### Title Bar
- Custom div with `WindowControlArea` hit-test regions (not gpui-component TitleBar)
- Accent blue title text, Unicode window control glyphs
- Close button: bright red (#e81123) hover, flush to corner

### Toolbar
- Left: drive dropdown (gpui-component Select) + Scan button + progress bar
- Right: WizTree-style drive info panel (Selection, Total/Used/Free space with %)
- Progress bar: empty when idle, partial blue when scanning, full accent blue when complete

### Tree View (9 columns)
- Name (flex-grow): indent + chevron + icon (💾/📁/📄) + name
- % of Parent: `scan_progress * 100` as "XX.X %"
- Color bar: width = scan_progress, color from SizeChange
- Size / Prev Size: `format_size()` values
- % of Previous: computed change percentage
- Files / Folders: `format_number()` counts
- Modified: timestamp string

### Color Coding (SizeChange)
- Decreased → `#22c55e` (green)
- Unchanged / NoBaseline → `#6b7280` (grey)
- SmallGrowth (0–50%) → `#eab308` (yellow)
- MediumGrowth (50–100%) → `#f97316` (orange)
- LargeGrowth (>100%) → `#ef4444` (red)

---

## Scanner Flow (`scanner.rs`)

1. User clicks "Scan Now" → `start_scan()`
2. `cx.background_executor().spawn(scan_dir_sync(root))` — depth-first recursive walk
   - Collects `file_count`, `folder_count`, `modified` per node
   - Sums children sizes bottom-up
3. On completion (foreground `cx.spawn`):
   - `persistence::save_scan()` to SQLite
   - Refresh scan history
   - Set `scan_completed = true`, rebuild flattened tree
   - `cx.notify()`

---

## Testing Strategy

56 tests across 5 modules, all using `#[cfg(test)]` blocks within source files.

- **models.rs**: SizeChange boundary cases, format_size, format_number
- **persistence.rs**: In-memory SQLite — schema migration, save/load roundtrip, filter by drive, delete cascade (includes file_count/folder_count/modified fields)
- **scanner.rs**: flatten_tree collapse/expand, merge_baseline, build_baseline_map, scan_dir_sync with tempfile::TempDir (verifies counts + modified)
- **drive_selector.rs**: 5 gpui tests (requires `gpui_component::init()` in test setup)
- **scan_history.rs**: 7 gpui tests
- **tree_view.rs**: 5 gpui tests

```bash
cargo test                        # all 56 tests
cargo clippy -- -D warnings       # must be clean
```

---

## What's Next

1. **Manual verification** — `cargo run`, test drive selection, scanning, tree expand/collapse
2. **Merge PR #7** — Phase 6 UI restructure into main
3. **Future enhancements** (not yet planned):
   - Settings panel (gear icon is placeholder)
   - VirtualList for large trees (currently renders all rows)
   - Scan progress callbacks (currently indeterminate bar)
   - Export/report functionality
   - Keyboard navigation in tree view
