## Summary
Checks that an edit request has a path, coordinates, contents, and version compatible with its operation.

## Behavior
Returns `Err` with a specific message for an empty path, invalid coordinates, contents supplied for a deletion, or a missing or invalid required version. Lines start at 1; versions must contain 64 hexadecimal characters. Returns `Ok(())` if all checks pass.

## Imports
- `super::{Operation, Request}`: Operation and validated-request types.
