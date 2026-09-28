## Summary
Converts the value `allow` to `true` during deserialization.

## Behavior
Deserializes the input as text and returns `Ok(true)` only when it equals `allow`; any other value produces an error stating that `TREE_TOOL` must be `allow`.

## Imports
- `serde`: Deserializes the input and creates the custom error.
