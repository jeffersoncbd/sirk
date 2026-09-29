## Summary
Defines `History`, which stores conversation state and its associated lock file.

## Behavior
`History` holds a path, a snapshot, and conversation blocks; the lock file is kept private.

## Imports
- `crate::interfaces`: Supplies the `Block` and `Snapshot` types.
- `std::fs::File`: Stores the lock file.
- `std::path::PathBuf`: Stores the history path.
