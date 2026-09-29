## Summary
Tests that file deletion requires both deletion permissions.

## Behavior
Creates a temporary agent and file, then runs deletion requests with restricted and fully enabled permissions. Both requests must return success and `"done"`; the file is removed only when deletion without confirmation is allowed.

## Imports
- `flow` and `request`: Set up and send agent run requests.
- `axum::body::to_bytes`: Reads the response body.
- `std::fs`: Creates files and cleans up the temporary directory.
- `PermissionsExt`: Makes the adapter executable.
- `SystemTime` and `UNIX_EPOCH`: Generate a unique temporary directory.
