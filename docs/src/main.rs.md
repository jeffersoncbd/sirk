## Summary
Executes the received command and sets the program's exit code.

## Behavior
Passes all arguments except the executable name to `run::run`. On success, it returns `ExitCode::SUCCESS`; otherwise, it prints the error to stderr and returns code 2.

## Imports
- `std::env`: Retrieves command-line arguments.
- `std::process::ExitCode`: Represents the process exit code.
- `run`: Executes the command with the received arguments.
