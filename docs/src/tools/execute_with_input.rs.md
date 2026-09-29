## Summary
Executes the named tool for a directory and returns its output.

## Behavior
`TREE` lists and formats the directory’s files; `READ` returns the requested page content. Propagates operation errors and returns an error for unknown tool names.

## Imports
- `std::path::Path`: Represents the execution directory.
- `format_paths`: Formats the `TREE` file list.
- `read`: Retrieves requested page content.
- `Tree`: Lists files in the directory.
