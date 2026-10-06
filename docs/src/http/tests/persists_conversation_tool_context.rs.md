## Summary
Verifies that tool output persists across requests in the same conversation.

## Behavior
Creates a temporary agent and fact file, then sends two requests with the same conversation ID. It checks that the first response asks a question, the second uses the stored fact, and the conversation history contains the tool result, then removes the temporary directory.

## Imports
- `super::{flow, request}`: Starts the flow and sends HTTP requests.
- `axum::body::to_bytes`: Reads response bodies.
- `serde_json::Value`: Parses response and conversation JSON.
- `std::fs`: Creates, reads, and removes temporary files.
- `PermissionsExt`: Makes the adapter script executable.
- `SystemTime`, `UNIX_EPOCH`: Generates a unique temporary directory name.
