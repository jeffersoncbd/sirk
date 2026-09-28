## Summary
Lists existing, non-ignored files beneath a Git working directory.

## Behavior
Resolves and validates the directory, queries Git for tracked and untracked files, and applies `.treeignore` rules. Converts paths while preserving non-UTF-8 bytes, skips deleted files and entries that are not files, includes symbolic links without following them, and returns sorted, deduplicated paths. Resolution, Git execution, or file-inspection errors are returned as `String`.

## Imports
- `crate::services`: Runs Git commands with structured arguments.
- `std::collections::BTreeSet`: Stores excluded paths.
- `std::fs`: Inspects metadata without following links.
- `std::io`: Discards process output and identifies missing files.
- `std::path::Path`: Represents the supplied directory.
- `super::Tree`: Builds the result with its root and files.
- `super::path::path_from_bytes`: Converts paths while preserving bytes.
