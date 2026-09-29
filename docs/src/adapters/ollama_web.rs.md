## Summary
Builds non-streaming Ollama API invocations and parses their responses.

## Behavior
`invocation` requires a model, rejects event streams, and creates a POST request with the prompt and model. It adds authorization from the configured API key when available and resolves the endpoint. `response` extracts the response text and token usage when both counts are present, or returns an error for invalid JSON.

## Imports
- `api_key`, `endpoint`: Resolve authentication and request URL.
- `BTreeMap`: Stores invocation environment variables.
- `Deserialize`: Parses the API response JSON.
- `json`: Builds the request body.
- `crate::harness`: Adapter, invocation, response, and error types.
