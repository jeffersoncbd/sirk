## Summary
Defines mutable conversation-history state and its storage structure.

## Behavior
Declares helper modules, re-exports `Block` and `Snapshot`, and defines `History`, which holds the path, snapshot, blocks, and a lock file.

## Imports
- `crate::interfaces`: Provides `Block` and `Snapshot`.
- `std::fs::File`: Holds the lock file.
- `std::path::PathBuf`: Stores the history path.
