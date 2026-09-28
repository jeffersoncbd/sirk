## Summary
Builds an OpenCode invocation for a harness request.

## Behavior
Starts with the `run` command and sets a fixed title without enabling automatic approval. Includes JSON format when event streaming is enabled and a model when supplied; then separates the prompt from the arguments with `--`. Returns the program, arguments, and request working directory, without additional environment variables.

## Imports
- `super::OpenCodeAdapter`: Adapter that provides the executable.
- `HarnessAdapter`: Defines the invocation implementation.
- `HarnessError`: Harness error type.
- `Invocation`: Process-call structure.
- `RunRequest`: Contains request options and data.
