## Summary
`page` reads a requested range of lines from a file under the given directory.

## Behavior
It parses the input request, reads the requested file, and returns up to `limit` lines starting at the one-based `offset`. It returns an error if parsing or reading fails, or if the offset is beyond the end of a nonempty file.

## Imports
- `super::request::request`: Parses the input into a file path and line range.
- `std::path::Path`: Represents the directory path.
