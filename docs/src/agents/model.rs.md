## Summary
Deserializes an optional model, trimming whitespace and converting it to lowercase.

## Behavior
Converts the input to `Option<String>` and propagates deserialization errors; when a string is present, normalizes it with `trim` and `to_lowercase`.

## Imports
- `serde::Deserialize`: Enables string deserialization.
