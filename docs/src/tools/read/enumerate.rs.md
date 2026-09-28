## Summary
`enumerate` prefixes each content line with a stable number starting at 1.

## Behavior
Starts output with the `Line | Content` header and iterates over lines while preserving their original terminators. Prefixes each line with its number and returns the resulting text.

## Imports
- `std` (prelude): Provides `String` and text operations.
