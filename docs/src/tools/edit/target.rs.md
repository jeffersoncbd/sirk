## Summary
Resolves an edit target path and ensures it stays within the execution directory.

## Behavior
Rejects empty paths and paths with non-normal components. Checks parent components for non-directory paths and disallows symlinks; when `allow_missing` is true, accepts missing targets or parents. For existing files, canonicalizes the target and rejects paths outside the root. Returns the resolved path or an error.

## Imports
- `std::fs`: Inspects path metadata and detects symlinks.
- `std::path`: Validates, builds, and canonicalizes paths.
