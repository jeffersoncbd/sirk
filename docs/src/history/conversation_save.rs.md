## Summary
`save` writes the conversation data as pretty-printed JSON to its path.

## Behavior
It serializes the data, adds a trailing newline, writes and syncs a temporary file, then renames it into place and syncs the parent directory. Serialization and filesystem errors are returned as `String`.

## Imports
- `super::Conversation`: Provides the conversation data and destination path.
- `serde_json`: Serializes conversation data as pretty-printed JSON.
- `std::fs`: Opens and renames files.
- `std::io::Write`: Writes serialized bytes to the temporary file.
