## Summary
Defines the request, response, error, and adapter interface for running coding-agent CLIs.

## Behavior
`HarnessAdapter` identifies an adapter and converts a `RunRequest` into an `Invocation`, returning `HarnessError` on failure. Its default `response` wraps stdout as response text with no token usage.

## Imports
- `crate::interfaces::Invocation`: Represents a CLI invocation.
- `std::path::PathBuf`: Stores the request’s working directory.
