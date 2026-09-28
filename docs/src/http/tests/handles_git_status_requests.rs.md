## Summary
Tests whether the Git status route returns visible files from the directory.

## Behavior
The test creates a temporary repository, adds visible files and a file ignored by `.treeignore`, and sends a `POST` request to `/v1/git/status`. It checks that the response has status 200 and contains only `.treeignore` and `visible.rs`, then removes the temporary directory.

## Imports
- `handle`: Executes the simulated HTTP request.
- `BashService`, `Invocation`: Initialize the Git repository.
- `fs`, `io`: Create files and discard command output.
- `SystemTime`, `UNIX_EPOCH`: Make the temporary directory name unique.
- `#[test]`: Marks the function as a test.
