## Summary
Commits the prepared edit to its target file and returns the resulting diff.

## Behavior
Validates the edit and computes the new contents. It returns an error if the target matches neither the prepared nor resulting contents, or changes during preparation. Otherwise, it synchronizes an already-applied result or writes and syncs a temporary file, then publishes it while preserving existing permissions and avoiding overwriting a concurrently created target. Temporary files are removed on failure.

## Imports
- `Pending`: Provides the edit request and commit state.
- `read_optional`: Reads the target when it exists.
- `sync_parent`: Synchronizes the target's parent directory.
- `target`: Resolves the edit's target path.
- `std::fs`: Creates, inspects, and publishes files.
- `std::io::Write`: Writes the new file contents.
- `std::path::Path`: Represents the input directory.
- `std::sync::atomic`: Generates unique temporary names.
