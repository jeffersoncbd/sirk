## Summary
Runs a Git command in the specified directory and returns stdout as bytes.

## Behavior
Builds a `git` invocation with the supplied arguments and default environment. Converts execution failures into contextual `Err` values; returns `stdout` on success or reports the exit status on failure.

## Imports
- `BashService`: Executes the process and captures its output.
- `Invocation`: Defines the program, arguments, directory, and environment.
- `std::io`: Provides a sink for other output.
- `std::path::Path`: Represents the working directory.
