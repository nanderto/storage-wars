# models

Core data model types for the disk analyzer desktop application.

## Overview

This crate provides the fundamental data structures shared across the
application:

| Type | Description |
|------|-------------|
| `FsNode` | Filesystem tree node with name, path, size, prev_size, file/folder counts, and modified timestamp |
| `DbNode` | Flat database representation of an `FsNode` with `parent_id` for relational storage |
| `ScanMeta` | Metadata for a scan session (root path, timestamps, totals, completion state) |
| `DriveInfo` | Drive/volume descriptor with name, volume label, mount point, and space information |
| `UiNode` | `FsNode` wrapper with tree depth, expansion state, and optional scan progress |
| `SizeChange` | Delta classification (New / Deleted / IncreasedLarge / IncreasedSmall / DecreasedSmall / DecreasedLarge / Unchanged) with hex colors |
| `ScanMessage` | Enum of messages emitted during scanning: `DirScanned`, `ScanError`, `Complete` |

## Building