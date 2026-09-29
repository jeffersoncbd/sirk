## Summary
Builds a non-streaming NVIDIA API invocation from a run request.

## Behavior
Requires a model and rejects event streams, returning `HarnessError` for either condition. It retrieves the API key for the request’s working directory, then returns an invocation configured for a JSON POST with the model and prompt, endpoint, executable, and authorization environment variable.

## Imports
- `std::collections::BTreeMap`: Stores invocation environment variables.
- `serde_json::json`: Builds the JSON request body.
- `super::{ENDPOINT, NvidiaApiAdapter}`: Supplies the endpoint and adapter.
- `crate::harness`: Provides invocation, request, adapter, and error types.
