## Summary
Converts bytes to a `PathBuf`, respecting platform capabilities.

## Behavior
On Unix, preserves path bytes directly. On other platforms, requires UTF-8 and returns an error message if conversion fails.

## Imports
- `std::path::PathBuf`: Represents the converted path.
- `std::os::unix::ffi::OsStrExt`: Converts bytes to `OsStr` on Unix.
- `std::str`: Validates UTF-8 on non-Unix platforms.
- `std::ffi::OsStr`: Creates `OsStr` from bytes on Unix.
- `format!`: Builds the error message.
