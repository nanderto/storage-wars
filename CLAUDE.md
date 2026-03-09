# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build          # debug build
cargo build --release
cargo run
cargo check          # fast type-check without linking
```

This project requires **Rust 1.93**. If the default toolchain is older, add a `rust-toolchain.toml`:
```toml
[toolchain]
channel = "1.93"
```

## Project Intent

**Storage Wars** is a desktop GUI application (Windows-first) for tracking disk folder growth over time. Key design decisions:

- **UI framework**: `gpui` (from the Zed editor) + `gpui-component`
- **Storage**: SQLite via `rusqlite` (bundled — no system dependency)
- **Drive enumeration**: `sysinfo`
- **Scan trigger**: Manual only — user clicks "Scan Now"
- **History**: Full scan history kept; any two scans can be compared
- **Tree default**: Starts fully collapsed (top-level folders only)

## Planned Module Structure

```
src/
  main.rs           - App entry point, window creation
  app_view.rs       - Root view: drive selector + scan history panel + tree area
  drive_selector.rs - Drive list panel (all available drives, pick one)
  scan_history.rs   - Scan history list + "pick two to compare" UI
  tree_view.rs      - Hierarchical file/folder tree with color-coded status bars
  scanner.rs        - Synchronous recursive directory scanner (run on background thread)
  persistence.rs    - SQLite open/migrate/save/query
  models.rs         - All shared data types
```

## Git Workflow

Every set of changes must follow this workflow:

1. Create a feature branch off `main`: `git checkout -b phase-N-description`
2. Commit changes to that branch
3. Push the branch and open a pull request against `main`
4. Do not merge or push directly to `main`

Branch naming convention: `phase-1-data-layer`, `phase-2-scanner`, `phase-3-ui-components`, etc.

## Implementation Plan

See `Tasks.md` for the phased checklist. Phases must be completed and reviewed one at a time:

- **Phase 0** — Project initialization ✅
- **Phase 1** — Data layer (`Cargo.toml` deps, `models.rs`, `persistence.rs`)
- **Phase 2** — Scanner (`scanner.rs`)
- **Phase 3** — UI components (`drive_selector.rs`, `scan_history.rs`, `tree_view.rs`)
- **Phase 4** — App wiring (`app_view.rs`, `main.rs`)
- **Phase 5** — Build & fix

**Important**: Do not proceed to the next phase without explicit user approval.

## Key Architecture Notes

- `AppState` (in `app_view.rs`) is the single source of truth — drives, selected drive, scan history, compare selections, flattened tree nodes, scanning flag.
- The tree is stored as a nested `Vec<FsNode>` and flattened into `Vec<UiNode>` for rendering. Expand/collapse mutates the `expanded_paths: HashSet<PathBuf>` and re-flattens.
- Scans are run via `std::thread::spawn` (blocking), then results are sent back to the UI thread via `cx.spawn`.
- SQLite DB path: `{APPDATA}/storage-wars/storage-wars.db`
- Size-change color coding: green=decreased, yellow=small growth (<50%), orange=medium (50–100%), red=large (>100%), grey=no baseline.
