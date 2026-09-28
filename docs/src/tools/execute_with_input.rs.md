## Summary
Runs the requested tool in the specified directory and returns its result.

## Behavior
For `TREE`, lists directory files and formats their paths; for `READ`, reads the requested contents. Errors from these operations are propagated as `String`; unknown names return an error.

## Imports
- `std::path::Path`: Represents the execution directory.
- `format_paths`: Formats the path list.
- `read`: Reads the requested contents.
- `Tree`: Lists directory files.
