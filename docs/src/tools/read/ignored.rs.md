## Summary
Checks whether a file should be ignored according to the root `.readignore` rules.

## Behavior
If `.readignore` does not exist, returns `Ok(false)`; if it cannot be read, returns `Err("AccessDenied")`. Normalizes the relative path and returns `Ok(true)` when it finds a matching pattern that is neither empty nor a comment.

## Imports
- `std::fs`: Reads the `.readignore` file.
- `std::io`: Identifies when the file does not exist.
- `std::path::Path`: Represents the supplied paths.
- `super::ignore`: Compares the path with exclusion patterns.
