## Summary
Runs an agent request from an HTTP request body and returns its result.

## Behavior
Rejects invalid UTF-8 or JSON with `400 Bad Request`. Runs the agent service in a blocking task, returning the result as JSON on success or a `500 Internal Server Error` with an error message if execution fails.

## Imports
- `super::super::types`: Request and JSON response types
- `axum`: HTTP body, status, and response types
- `serde_json`: Builds JSON response bodies
