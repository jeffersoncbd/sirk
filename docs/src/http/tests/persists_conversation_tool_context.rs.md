## Summary
Tests that a conversation retains tool output between agent requests.

## Behavior
Creates a temporary agent and fact file, then sends two requests using the same conversation ID. It checks that the first response asks a question, the second uses the stored fact, and the saved conversation includes the tool result; finally, it removes the temporary directory.

## Imports
- `super::{flow, request}`: Starts the flow and sends HTTP requests.
- `axum::body::to_bytes`: Reads response bodies.
- `serde_json::Value`: Parses JSON responses and saved history.
- `std::{fs, PermissionsExt, SystemTime, UNIX_EPOCH}`: Creates test files and timestamps.
