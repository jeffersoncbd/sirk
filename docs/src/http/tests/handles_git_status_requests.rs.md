## Summary
Tests that the Git status endpoint excludes files ignored by `.treeignore`.

## Behavior
The test creates a temporary Git repository with visible and ignored files, sends a request to `/v1/git/status`, and asserts a successful response containing only `.treeignore` and `visible.rs`. It removes the temporary directory afterward.

## Imports
- `request`: Sends the simulated HTTP request.
- `BashService`, `Invocation`: Initialize the temporary Git repository.
- `to_bytes`: Reads the response body.
- `fs`, `io`: Create files and discard command output.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary directory name.
