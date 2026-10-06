## Summary
Deserializes the string `"allow"` as `true`.

## Behavior
Reads a string from the deserializer and returns `Ok(true)` only when it equals `"allow"`; otherwise, returns a custom deserialization error.

## Imports
- `serde`: Provides deserialization traits and custom error creation.
