## Summary
`deserialize` converts the string `"allow"` into `true`.

## Behavior
It deserializes a string and returns `Ok(true)` only when the value is exactly `"allow"`; otherwise it returns a custom error stating that `ASK_TOOL` must be `allow`.

## Imports
- `serde`: Provides string deserialization and custom errors.
