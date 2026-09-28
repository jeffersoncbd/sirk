## Summary
Tests whether an HTTP request stages a file in the supplied directory.

## Behavior
Creates a temporary repository, writes `generated.md`, and sends a `POST` request to `/v1/git/add`. Checks for HTTP status 200 and a `status` field equal to `ok`, verifies that the file was staged, and removes the temporary directory. Setup or execution errors fail the test.

## Imports
- `handle`: Processes the test HTTP request.
- `BashService`, `Invocation`: Run Git commands in the test directory.
- `std::fs`: Creates, writes, and removes files and directories.
- `std::io`: Provides a sink for command output.
- `SystemTime`, `UNIX_EPOCH`: Make the temporary directory name unique.
