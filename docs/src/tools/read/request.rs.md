## Summary
Parses a READ input into a file path, starting line, and line limit.

## Behavior
Inputs that do not start with `{` become paths with offset 1 and an unlimited line limit. JSON inputs accept `path` or `filePath`, default offset to 1 and limit to 2,000, reject unknown fields and zero values, and return parsing or validation errors as `String`.

## Imports
- `serde::Deserialize`: Enables deserialization of JSON arguments.
- `serde_json`: Parses structured READ input.
