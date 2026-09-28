## Summary
Reads a UTF-8 file within the execution directory while enforcing access rules.

## Behavior
Rejects empty paths, directories, and paths escaping the root; returns errors if it cannot resolve, validate, or read the file. Before reading, checks whether the path is ignored and returns `AccessDenied` if so.

## Imports
- `std::fs`: Reads file contents.
- `std::path::Path`: Represents and validates paths.
- `super::ignored`: Checks whether the file is blocked.
