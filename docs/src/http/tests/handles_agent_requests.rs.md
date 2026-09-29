## Summary
Verifies that an agent request runs over HTTP and returns its output.

## Behavior
Creates a temporary agent with an executable adapter, sends a `POST` request to `/v1/agent/run`, and checks for a successful response containing the adapter’s output. Removes the temporary directory afterward.

## Imports
- `request`: Sends the HTTP request.
- `axum::body::to_bytes`: Reads the response body.
- `serde_json::Value`: Parses the JSON response.
- `std::fs`: Creates and removes temporary files and directories.
- `PermissionsExt`: Makes the adapter executable.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary directory name.
