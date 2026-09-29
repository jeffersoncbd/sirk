## Summary
Tests that template-like input is passed to the agent as literal text.

## Behavior
Creates a temporary agent with an adapter that succeeds only if it receives `{{ outputs.plan }}` unchanged, sends the input through the HTTP endpoint, and checks the response status and result. Removes the temporary directory afterward.

## Imports
- `request`: Sends the HTTP request under test.
- `axum::body::to_bytes`: Reads the response body.
- `std::fs`: Creates files and removes the temporary directory.
- `PermissionsExt`: Makes the adapter executable.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary directory name.
