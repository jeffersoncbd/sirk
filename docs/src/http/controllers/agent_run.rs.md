## Summary
Runs the requested agent and returns its result or question as JSON.

## Behavior
Rejects invalid JSON or a flow ID that is missing, invalid, unknown, or for another directory with `400 Bad Request`. Runs the agent in a blocking task, returning either its result or an `ask` question; agent or task failures return `500 Internal Server Error`.

## Imports
- `JsonResponse`: Wraps response bodies as JSON
- `schemas::requests`: Provides the agent run request type
- `schemas::responses`: Provides agent result and error types
- `axum`: Provides HTTP request and response types
- `tokio`: Runs the agent service in a blocking task
