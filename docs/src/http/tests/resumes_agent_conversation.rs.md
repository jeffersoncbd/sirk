## Summary
No non-test function is present to document; the file contains only a test.

## Behavior
The test exercises resuming an agent conversation from persisted messages.

## Imports
- `super::{flow::flow, request::request}`: Test HTTP flow setup and requests
- `axum::body::to_bytes`: Reads response bodies
- `serde_json::Value`: Parses JSON responses and history
- `std`: Creates temporary files and directories for the test
