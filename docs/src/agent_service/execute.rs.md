## Summary
Executes an invocation and returns stdout only if the process succeeds.

## Behavior
Executes `invocation` while discarding additional output; execution errors are converted to text. If the process fails, returns a message containing the program and status without accepting its output. On success, returns `stdout`.

## Imports
- `crate::services`: Provides the process executor and invocation.
- `std::io`: Provides a sink that discards additional output.
