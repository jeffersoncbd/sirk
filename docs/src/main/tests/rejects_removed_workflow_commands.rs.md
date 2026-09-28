## Summary
Verifies that `run` rejects invalid or removed commands.

## Behavior
Runs `run` with the arguments `run documentation`, `resume history/run-old.log`, and `rpc`; each case is expected to return an error containing `invalid command`.

## Imports
- `super::*`: Imports `run` from the parent module.
