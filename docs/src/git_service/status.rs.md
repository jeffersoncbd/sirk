## Summary
Lists Git-changed paths within the requested directory, excluding ignored files.

## Behavior
Locates the project and runs `git status` and `git ls-files` to obtain changes and ignored paths. Validates records, handles renames and copies, converts UTF-8 paths, and removes the requested-directory prefix. Keeps deleted files or files still visible in the filesystem; returns an error if commands fail or output is invalid. Sorts and deduplicates paths before returning them.

## Imports
- `super::project::project`: Locates the Git project and its prefix.
- `super::run::run`: Runs Git commands and returns output.
- `std::collections::BTreeSet`: Stores deleted paths.
- `std::fs`: Checks path existence and type.
- `std::path::Path`: Represents the input directory.
