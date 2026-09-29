## Summary
Builds non-streaming OpenRouter chat requests and extracts response text and token usage.

## Behavior
`invocation` requires a model, rejects event streams, and returns a POST invocation with the prompt, endpoint, and API key supplied through the environment; configuration errors propagate. `response` parses the JSON, returns the first choice’s content and token usage when both counts are present, and reports invalid JSON or missing choices as errors.

## Imports
- `std::collections::BTreeMap`: Stores invocation environment variables.
- `serde::Deserialize`: Deserializes the API response.
- `serde_json::json`: Builds the request body.
- `crate::harness`: Provides adapter types, requests, and errors.
