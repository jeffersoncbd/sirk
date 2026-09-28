## Summary
Formats file paths as a pretty-printed JSON array.

## Behavior
Converts each path to UTF-8, returning an error naming the tool if a path cannot be represented. Serializes the paths with a trailing newline, reporting serialization errors with the tool name.

## Imports
- `std::path::PathBuf`: Type of the input paths.
- `serde_json`: Serializes paths as JSON.
