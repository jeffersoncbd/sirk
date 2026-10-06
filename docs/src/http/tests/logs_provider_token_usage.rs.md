## Summary
No production function is present; the file contains only a test.

## Behavior
The test checks that provider token usage is recorded in flow and aggregate logs while the API response returns the result without token counts or a conversation ID.

## Imports
- `super::{flow, request}`: Set up a flow and send API requests.
- `axum::body::to_bytes`: Read the HTTP response body.
- `std`: Create test files, set permissions, and generate a unique directory.
