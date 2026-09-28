## Summary
Reads a UTF-8 file if it exists.

## Behavior
Returns the contents in `Some`, `None` if the file does not exist, or a descriptive error if reading fails.

## Imports
- `std::fs`: Reads the file.
- `std::path::Path`: Represents the file path.
