# scanner

Multi-threaded filesystem scanner built with Rust `std::thread`.

## Architecture

| Function | Description |
|---|---|
| `scan_dir_incremental` | Spawns up to 8 worker threads pulling from a shared work queue (protected by a `Mutex` + `Condvar`). Sends `ScanMessage` (`DirScanned` / `ScanError` / `Complete`) to the caller via an `mpsc` channel. |
| `read_dir_immediate` | Reads one directory level and returns entries immediately (non-recursive). |
| `scan_dir_sync` | Recursive single-threaded scan with bottom-up size aggregation. |

All operations:
- Respect an `Arc<AtomicBool>` cancellation flag.
- Silently skip permission errors.

## Build