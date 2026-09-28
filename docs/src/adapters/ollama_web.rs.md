## Summary
Converts requests into non-streaming calls to the Ollama web API and extracts the response.

## Behavior
`invocation` requires a model, rejects requests with event streaming, and builds a POST call with the prompt and model; it includes authentication when a key is available and resolves the configured endpoint. `response` reads the JSON `response` field or returns an error if the contents are invalid.

## Imports
- `api_key`, `default`, `dotenv`, `endpoint`, `new`, `nonempty`: Adapter configuration and construction.
- `std::collections::BTreeMap`: Stores call environment variables.
- `serde::Deserialize`: Deserializes the JSON response.
- `serde_json::json`: Builds the request JSON body.
- `crate::harness`: Types and trait for harness calls.
