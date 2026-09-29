## Summary
Validates a pending edit record before it is accepted.

## Behavior
Validates the request first, then checks that missing-file records have empty prior contents and use `Append`, `Prepend`, or `Write`. If prior contents are present, verifies the operation can be applied to them. Returns the first error or `Ok(())`.

## Imports
- `super`: Provides `Operation` and `Pending`.
