## Summary
Synchronizes the parent directory of the supplied path to disk.

## Behavior
Gets the parent directory, opens it, and calls `sync_all`; converts open or synchronization failures to `String`. Expects the path to have a parent directory.

## Imports
- `std::fs::File`: Opens the directory for synchronization.
- `std::path::Path`: Represents the supplied path.
