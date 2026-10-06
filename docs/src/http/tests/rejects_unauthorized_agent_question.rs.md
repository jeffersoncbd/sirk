## Summary
Verifies that an agent question without permission is rejected with an HTTP 500 error.

## Behavior
Creates a temporary agent configuration whose executable outputs an `ASK:` request, sends an agent-run request, and checks the error response before removing the temporary directory.

## Imports
- `super::{flow, request}`: Creates a flow and sends the HTTP request.
- `axum::body::to_bytes`: Reads the response body.
- `serde_json::Value`: Parses the response JSON.
- `std::fs`: Creates and removes temporary files and directories.
- `std::os::unix::fs::PermissionsExt`: Makes the test adapter executable.
- `std::time`: Generates a unique temporary directory name.
- `tokio::test`: Runs the test asynchronously.
