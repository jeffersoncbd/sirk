## Summary
Tests that an HTTP agent run appends text to a file and returns its result.

## Behavior
Creates a temporary workspace with a fake adapter and editor agent, sends a run request, and checks the response and updated file contents before removing the workspace.

## Imports
- `flow`, `request`: Set up and send the simulated HTTP request.
- `axum::body::to_bytes`: Reads the response body.
- `std::fs`: Creates, reads, and removes workspace files.
- `PermissionsExt`: Makes the fake adapter executable.
- `SystemTime`, `UNIX_EPOCH`: Create a unique temporary directory name.
