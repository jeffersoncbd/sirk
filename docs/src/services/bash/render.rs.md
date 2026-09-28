## Summary
Builds a command line for executing an `Invocation`.

## Behavior
Adds `exec --`, quotes the program and each argument according to the invocation environment, and joins everything with spaces. Returns the line as a `String`.

## Imports
- `BashService`: Type that owns the method.
- `Invocation`: Provides the program, arguments, and environment.
- `shell_quote`: Quotes the program and arguments.
