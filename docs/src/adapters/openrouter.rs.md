## Summary
Adapts harness requests into non-streaming OpenRouter chat API calls and extracts response content.

## Behavior
`invocation` requires a model and rejects event streams; it then retrieves the API key and builds a POST call with the prompt, model, and authentication in the environment. Configuration errors are propagated. `response` parses the JSON and returns the first choice's content, or an error if the response is invalid or contains no choices.

## Imports
- `std::collections::BTreeMap`: Builds the call's environment variables.
- `serde::Deserialize`: Parses the JSON response.
- `serde_json::json`: Builds the request JSON body.
- `crate::harness`: Provides adapter types and errors.
