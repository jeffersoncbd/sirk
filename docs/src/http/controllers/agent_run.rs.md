## Summary
Runs an agent request and returns its result as an HTTP response.

## Behavior
Returns `400 Bad Request` if the JSON request cannot be parsed. Runs the agent service in a blocking task, returning the result on success or a `500 Internal Server Error` with an error message if execution or the task fails.

## Imports
- `json_response`: Wraps response bodies as JSON
- `schemas::requests`: Provides the agent request type
- `schemas::responses`: Provides result and error response types
- `axum`: Provides HTTP JSON, status, and response types
