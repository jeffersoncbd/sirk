## Summary
Parses standalone requests to read a file or inspect the file tree.

## Behavior
Trims surrounding whitespace, recognizes `TREE` or `READ:`, and returns the request type with its path. Returns `None` for other text or paths containing line breaks.

## Imports
- None: Uses only standard Rust methods and types.
