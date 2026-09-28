## Summary
Defines the types and interface for running coding agents through different adapters.

## Behavior
`HarnessAdapter` provides an identifier and converts a request into an invocation, returning errors for invalid options or configurations. By default, `response` passes stdout through unchanged. `RunRequest` holds execution data, and `HarnessError` represents integration failures.

## Imports
- `crate::interfaces::Invocation`: Represents the agent call.
- `std::path::PathBuf`: Stores the working directory.
