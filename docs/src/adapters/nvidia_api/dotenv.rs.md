## Summary
Reads a named nonempty value from a directory’s `.env` file.

## Behavior
Returns `Ok(None)` if the file or key is absent, and `Ok(Some(value))` for a matching nonempty value. File read or parse errors become `HarnessError::InvalidConfiguration`.

## Imports
- `std::io` and `std::path::Path`: Handle missing files and paths.
- `super`: Provides the adapter type and nonempty value helper.
- `crate::harness`: Provides the adapter trait and error type.
- `dotenvy`: Reads and parses `.env` files.
