//! # Scanner
//!
//! Multi-threaded filesystem scanner component.
//!
//! ## Overview
//!
//! Provides three scanning strategies:
//!
//! - [`scan_dir_incremental`]: Spawns up to 8 worker threads pulling from a shared work queue
//!   with condition variables. Sends [`ScanMessage`] events (`DirScanned`, `ScanError`,
//!   `Complete`) to a UI channel. Respects an atomic `cancelled` flag.
//!
//! - [`read_dir_immediate`]: Reads a single directory level and returns the entries immediately.
//!
//! - [`scan_dir_sync`]: Recursive single-threaded scanning with bottom-up size aggregation.
//!
//! Permission errors are silently skipped in all modes.

pub mod error;
pub mod message;
pub mod scanner;
pub mod worker;

pub use error::ScanError;
pub use message::ScanMessage;
pub use scanner::{read_dir_immediate, scan_dir_incremental, scan_dir_sync};