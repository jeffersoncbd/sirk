## Summary
Applies a prepared edit to the target file and returns its diff.

## Behavior
Validates the edit, calculates the final contents, and checks whether the file already contains that result; if so, synchronizes the file and its directory. Otherwise, requires the current contents to match the expected state, writes the result to a temporary file, and checks again for concurrent changes. Publishes the file without overwriting a target created during the operation, or replaces the existing file, synchronizes the directory, and removes the temporary file on error. Failures are returned as `Err`.

## Imports
- `Pending`: Type that owns the method.
- `read_optional`: Optional read of the target contents.
- `sync_parent`: Synchronization of the parent directory.
- `target`: Resolution of the target path.
- `std::fs`: File metadata and operations.
- `std::io::Write`: Writing temporary contents.
- `std::path::Path`: Represents the target directory.
- `std::sync::atomic`: Concurrent generation of temporary names.
