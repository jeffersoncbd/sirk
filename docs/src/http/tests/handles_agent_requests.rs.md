## Summary
Verifies that an HTTP agent request returns its output and records it in the conversation transcript.

## Behavior
Creates a temporary agent and executable adapter, sends a `POST` request to `/v1/agent/run`, and checks the status, response fields, and transcript contents. Removes the temporary directory afterward.

## Imports
- `flow`, `request`: Set up a flow and send the HTTP request.
- `axum::body::to_bytes`: Reads the response body.
- `serde_json::Value`: Parses the JSON response.
- `std::fs`: Creates, reads, and removes temporary files.
- `PermissionsExt`: Makes the adapter executable.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary directory name.
