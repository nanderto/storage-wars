# Storage Wars — Task Checklist

## Phase 0 — Project Initialization
- [x] `cargo init` in `C:\dev\Github\repos\storage-wars`
- [x] Create `.gitignore` (standard Rust: `/target`, `Cargo.lock`)
- [x] `git init`
- [ ] `git add .` + initial commit: `"chore: initialize storage-wars project"`
- [x] Write `Tasks.md` to project root

## Phase 1 — Data Layer
- [ ] Edit `Cargo.toml` — add all dependencies
- [ ] Create `src/models.rs`
- [ ] Create `src/persistence.rs`

## Phase 2 — Scanner
- [ ] Create `src/scanner.rs`

## Phase 3 — UI Components
- [ ] Create `src/drive_selector.rs`
- [ ] Create `src/scan_history.rs`
- [ ] Create `src/tree_view.rs`

## Phase 4 — App Wiring
- [ ] Create `src/app_view.rs`
- [ ] Edit `src/main.rs` — full entry point

## Phase 5 — Build & Fix
- [ ] `cargo build` — resolve compile errors
- [ ] Commit: `"feat: initial storage-wars implementation"`
