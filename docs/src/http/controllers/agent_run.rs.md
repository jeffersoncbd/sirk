## Summary
Runs an agent for a validated flow and returns its result or question as JSON.

## Behavior
Rejects invalid JSON or a missing, invalid, unknown, or directory-mismatched flow ID with `400 Bad Request`. Runs the agent in a blocking task; returns its result or `ask` response on success, and `500 Internal Server Error` if the agent or task fails.

## Imports
- `JsonResponse`: Wraps response bodies as JSON
- `schemas::requests`: Provides the agent run request type
- `schemas::responses`: Provides agent result and error types
- `axum`: Provides HTTP request and response types
- `tokio`: Runs the agent service in a blocking task
