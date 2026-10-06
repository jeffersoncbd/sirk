## Summary
This file contains no main function to document; it only defines a test.

## Behavior
The test checks that an authorized agent question is returned by the HTTP endpoint and recorded in the flow transcript and conversation history.

## Imports
- `super::{flow, request}`: Helpers used to exercise the HTTP endpoint
- `axum::body::to_bytes`: Reads the response body
- `serde_json::Value`: Parses JSON responses and conversation data
- `std`: Creates temporary files and directories for the test
- `tokio::test`: Runs the test asynchronously
