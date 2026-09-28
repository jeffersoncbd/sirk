## Summary
Contains a single test that verifies an agent can edit a file over HTTP.

## Behavior
The test creates a temporary directory, configures a fake adapter, and calls `handle` with a request to run the agent. It then checks the response and edited contents and removes the directory.

## Imports
- `handle`: Executes the simulated HTTP request.
- `std::fs`: Creates, reads, modifies, and removes test files.
- `PermissionsExt`: Marks the adapter as executable.
- `SystemTime` and `UNIX_EPOCH`: Generate a unique temporary directory name.
