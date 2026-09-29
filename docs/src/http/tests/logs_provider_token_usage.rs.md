## Summary
No production function is present; the file contains only a test.

## Behavior
The test checks that provider token usage is logged and accumulated when an agent run is resumed, while the HTTP response omits usage details.

## Imports
- `super::{flow, request}`: Test helpers for creating a flow and sending requests.
- `axum::body::to_bytes`: Reads the HTTP response body.
- `std`: Creates temporary files and manages permissions and timestamps.
