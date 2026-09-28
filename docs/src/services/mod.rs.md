## Summary
Exposes services and types related to command execution.

## Behavior
Declares the `bash` and `quote` modules and re-exports `Invocation`, `BashService`, and `ProcessOutput` for use by other modules.

## Imports
- `bash`: Process execution and output service.
- `quote`: Argument-handling module.
- `crate::interfaces::Invocation`: Re-exported type for building calls.
