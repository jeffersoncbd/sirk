## Summary
Defines edit request types and re-exports edit display and version functions.

## Behavior
`Operation` lists supported edits. `Request` carries the target path, operation, optional edit parameters, and input content; unknown fields are rejected during deserialization. `Pending` stores a request and the file’s prior content or missing-file state.

## Imports
- `serde`: Provides serialization and deserialization derives.
