## Summary
Validates a `Pending` record before accepting it.

## Behavior
Validates the request and returns its error, if any. If the file was missing, requires empty previous contents and an `Append` or `Prepend` operation; when previous contents exist, checks that the operation can be applied to them. Returns `Ok(())` if all checks pass.

## Imports
- `super`: Provides `Operation` and `Pending`.
