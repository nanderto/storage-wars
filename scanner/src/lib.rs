//! Multi-threaded filesystem scanner.
//!
//! # Overview
//!
//! - [`scan_dir_incremental`]: Spawns up to 8 worker threads pulling from a shared
//!   work queue with condition variables, sends [`ScanMessage`] to the UI channel.
//! - [`read_dir_immediate`]: Reads one directory level and returns entries immediately.
//! - [`scan_dir_sync`]: Recursive single-threaded scanning with bottom-up size aggregation.
//!
//! All scanning functions respect an atomic `cancelled` flag and silently skip
//! permission errors.

pub mod messages;
pub mod models;
pub mod scanner;
pub mod worker;

pub use messages::ScanMessage;
pub use models::DirEntry;
pub use scanner::{read_dir_immediate, scan_dir_incremental, scan_dir_sync};