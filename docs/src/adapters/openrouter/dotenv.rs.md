## Summary
Reads a nonempty key value from a directory's `.env` file.

## Behavior
Returns `Ok(None)` if `.env` does not exist; other read or parse errors become `HarnessError::InvalidConfiguration`. When the key is found, returns its value if nonempty; if it is not found, returns `Ok(None)`.

## Imports
- `super`: Access to the adapter and function that filters empty values.
- `crate::harness`: Types used to identify configuration errors.
- `std`: Missing-file error checks and directory paths.
- `dotenvy`: Reads and parses variables from `.env`.
