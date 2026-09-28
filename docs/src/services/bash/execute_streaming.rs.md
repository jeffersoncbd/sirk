## Summary
Executes an invocation and streams its output to standard output.

## Behavior
Calls `execute_to` with the received invocation and locked standard output, returning the result or I/O error.

## Imports
- `super`: `BashService` and `ProcessOutput` types.
- `crate::services::Invocation`: Invocation data.
- `std::io`: Standard output and I/O results.
