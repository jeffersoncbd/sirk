## Summary
Verifies that template-like input reaches the agent unchanged.

## Behavior
Creates a temporary agent whose adapter succeeds only when it receives `{{ outputs.plan }}`, sends that input to the HTTP endpoint, and checks for a successful response with the expected result. Removes the temporary directory afterward.

## Imports
- `flow::flow`: Creates the flow used by the request.
- `request::request`: Sends the HTTP request.
- `axum::body::to_bytes`: Reads the response body.
- `std::fs`: Creates files and removes the temporary directory.
- `PermissionsExt`: Makes the adapter executable.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary directory name.
