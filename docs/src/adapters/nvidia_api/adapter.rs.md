## Summary
Implements the harness adapter for the NVIDIA API.

## Behavior
The adapter identifies itself as `nvidia-api`, delegates invocation creation to `invocation::build`, and delegates stdout parsing to `response::parse`. Both operations return a `Result` with `HarnessError` on failure.

## Imports
- `crate::harness`: Adapter trait, request, invocation, response, and error types.
- `crate::adapters::nvidia_api`: NVIDIA API adapter and invocation/response handlers.
