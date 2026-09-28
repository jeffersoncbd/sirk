## Summary
Executes an invocation, captures stdout, and sends it to a destination.

## Behavior
Delegates execution and propagates errors, converts stdout to UTF-8 and returns an error if invalid, and preserves the status.

## Imports
- `super`: `BashService` and `ProcessOutput` types.
- `Invocation`: Execution data.
- `std::io`: Errors and stdout writing.
