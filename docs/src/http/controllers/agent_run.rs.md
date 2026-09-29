## Summary
Runs an agent request and returns its result as an HTTP response.

## Behavior
Rejects invalid JSON or flow IDs with `400 Bad Request`. Runs the agent service in a blocking task, returning its result on success or a `500 Internal Server Error` with an error message if execution fails.

## Imports
- `JsonResponse`: Wraps response bodies as JSON
- `schemas::requests`: Provides the agent request type
- `schemas::responses`: Provides result and error response types
- `axum`: Provides HTTP request and response types
- `tokio`: Runs the agent service in a blocking task

