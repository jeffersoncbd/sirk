## Summary
Verifies that deleting a file requires permission to delete without confirmation.

## Behavior
Creates a temporary agent and file, then submits a deletion request under restricted and fully enabled permissions. Both requests must succeed, but the file is removed only when both deletion permissions are enabled.

## Imports
- `request`: Sends the agent run request.
- `axum::body::to_bytes`: Reads the response body.
- `std::fs`: Creates and removes files and directories.
- `PermissionsExt`: Makes the test adapter executable.
- `SystemTime` and `UNIX_EPOCH`: Generate a unique temporary directory.
