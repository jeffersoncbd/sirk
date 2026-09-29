## Summary
Applies a validated edit request to text and returns the result or an error.

## Behavior
Checks the request and optional version, then handles writes and string replacements directly. Other operations calculate byte boundaries from line numbers, reject ranges beyond EOF, and combine the input with the unchanged text.

## Imports
- `super::{Operation, Request, version}`: Edit types and version calculation.
