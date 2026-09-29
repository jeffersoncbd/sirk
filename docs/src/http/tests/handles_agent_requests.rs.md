## Summary
Checks that an agent request runs over HTTP and returns its output.

## Behavior
Creates a temporary agent and executable adapter, sends a `POST` request to `/v1/agent/run`, and verifies the response and recorded transcript contain the expected output. Removes the temporary directory afterward.

## Imports
- `flow`, `request`: Set up the flow and send the HTTP request.
- `axum::body::to_bytes`: Reads the response body.
- `serde_json::Value`: Parses the JSON response.
- `std::fs`: Creates, reads, and removes temporary files.
- `PermissionsExt`: Makes the adapter executable.
- `SystemTime`, `UNIX_EPOCH`: Create a unique temporary directory name.
