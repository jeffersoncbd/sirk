## Summary
Defines data types for requesting and recording file edits.

## Behavior
`Operation` enumerates the available operations; `Request` holds the path, operation, parameters, and edit content. `Pending` stores the request and the file's previous state, including whether the file did not exist.

## Imports
- `serde`: Serializes and deserializes the defined types.
