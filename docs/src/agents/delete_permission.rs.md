## Summary
Converts the serialized value `"allow"` to `true`.

## Behavior
Deserializes a string and returns `Ok(true)` only when it is `"allow"`; any other value produces a descriptive error.

## Imports
- `serde`: String deserialization and custom error creation.
