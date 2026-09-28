## Summary
Contains a single test that verifies an agent can run over HTTP.

## Behavior
The test creates a temporary agent with an executable adapter, sends a `POST` request to `/v1/agent/run`, and checks the response status and result.

## Imports
- `handle`: Processes the HTTP request in the test.
- `serde_json`: Creates and parses JSON data.
- `std::fs`: Creates and removes temporary files and directories.
- `PermissionsExt`: Makes the adapter executable.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary name.
