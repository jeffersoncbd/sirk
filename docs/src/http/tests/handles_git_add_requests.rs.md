## Summary
Tests that `/v1/git/add` stages a file in the requested directory.

## Behavior
Creates a temporary Git repository and file, then posts the directory to the endpoint. It asserts a successful response and confirms the file is staged; setup or execution errors fail the test.

## Imports
- `flow`, `request`: Create a flow and send the HTTP request.
- `BashService`, `Invocation`: Run Git commands in the repository.
- `axum::body::to_bytes`: Reads the response body.
- `std::fs`: Creates, writes, and removes files and directories.
- `std::io`: Provides a sink for command output.
- `SystemTime`, `UNIX_EPOCH`: Make the temporary directory name unique.
