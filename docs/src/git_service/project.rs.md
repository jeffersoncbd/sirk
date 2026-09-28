## Summary
Resolves a Git directory and returns its root and relative-prefix paths.

## Behavior
Canonicalizes the supplied path, confirms it is a directory, and queries Git for the project root and prefix. Resolution, Git-command, or UTF-8 conversion errors are returned as `String`; trailing newlines are removed before constructing `Project`.

## Imports
- `super::run::run`: Runs Git commands in the supplied directory.
- `std::path::{Path, PathBuf}`: Represents and manipulates paths.
