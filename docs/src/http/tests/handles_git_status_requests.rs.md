## Summary
Tests that `/v1/git/status` omits files ignored by `.treeignore`.

## Behavior
Initializes a temporary Git repository with visible and ignored files, posts its directory to the status endpoint, and asserts a 200 response listing only `.treeignore` and `visible.rs`. Removes the temporary directory afterward.

## Imports
- `flow`, `request`: Set up and send the HTTP request.
- `BashService`, `Invocation`: Initialize the temporary Git repository.
- `to_bytes`: Read the response body.
- `fs`, `io`: Create files, discard command output, and clean up.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary directory name.
