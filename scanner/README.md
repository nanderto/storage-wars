# scanner

Multi-threaded filesystem scanner component written in Rust using `std::thread`.

## Architecture

The crate exposes three scanning strategies:

| Function | Strategy | Threads |
|---|---|---|
| `read_dir_immediate` | Single directory level | 1 (caller) |
| `scan_dir_sync` | Recursive, bottom-up size aggregation | 1 (caller) |
| `scan_dir_incremental` | Work-queue driven, sends `ScanMessage` events | up to 8 |

### `scan_dir_incremental`