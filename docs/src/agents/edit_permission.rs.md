## Summary
Converts the text value `allow` to `true`.

## Behavior
Deserializes a string and returns `Ok(true)` when it is `allow`; otherwise, it returns an error indicating the expected value.

## Imports
- `serde::Deserialize`: Deserializes the value as a string.
