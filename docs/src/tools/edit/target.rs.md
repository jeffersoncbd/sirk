## Summary
Resolves and validates an edit target path within the execution directory.

## Behavior
Canonicalizes the root directory and checks the requested path. Accepts existing regular files and rejects symbolic links and paths outside the root. If the file does not exist and `allow_missing` is true, requires the parent directory to exist and remain inside the root. Returns the resolved path or an error message.

## Imports
- `std::fs`: Queries path metadata.
- `std::path`: Manipulates and canonicalizes paths.
