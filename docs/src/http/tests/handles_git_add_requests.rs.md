## Summary
Verifies that the `/v1/git/add` endpoint stages a file in the requested directory.

## Behavior
Creates a temporary Git repository, writes `generated.md`, and sends a POST request with the directory path. It asserts a successful response with status `ok`, checks that the file is staged, and removes the temporary directory; setup or execution errors fail the test.

## Imports
- `request`: Sends the test HTTP request.
- `BashService`, `Invocation`: Run Git commands in the temporary repository.
- `axum::body::to_bytes`: Reads the response body.
- `std::fs`: Creates, writes, and removes files and directories.
- `std::io`: Provides a sink for command output.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary directory name.
