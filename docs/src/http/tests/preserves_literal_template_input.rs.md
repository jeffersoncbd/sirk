## Summary
Verifies that `{{ outputs.plan }}` is preserved as literal text in the input sent to the agent.

## Behavior
Creates a temporary agent with an adapter that accepts the literal expression, sends the input through the HTTP route, and checks for a `200` response with the expected result. It removes the temporary directory afterward.

## Imports
- `handle`: Runs the HTTP request under test.
- `std::fs`: Creates temporary files and directories.
- `PermissionsExt`: Sets executable permission on the adapter.
- `SystemTime`, `UNIX_EPOCH`: Generate a unique temporary name.
- `serde_json`: Builds the request and parses the response.
