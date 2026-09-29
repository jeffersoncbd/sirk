## Summary
Verifies that an HTTP agent run appends text to a file and returns its result.

## Behavior
Creates a temporary workspace with a fake executable adapter and editor agent, then sends a request to run the agent. It checks for a successful response, the expected result, and the updated file contents, then removes the workspace.

## Imports
- `request`: Sends the simulated HTTP request.
- `axum::body::to_bytes`: Reads the response body.
- `std::fs`: Creates, reads, and removes workspace files.
- `PermissionsExt`: Makes the fake adapter executable.
- `SystemTime`, `UNIX_EPOCH`: Create a unique temporary directory name.
