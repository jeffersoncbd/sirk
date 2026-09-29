## Summary
`NvidiaApiAdapter` configures access to NVIDIA’s chat completions API.

## Behavior
The adapter stores an executable and an optional API key; its endpoint is NVIDIA’s chat completions URL. The macro supplies the adapter implementation.

## Imports
- `adapter`: Provides the harness adapter implementation macro.
- `api_key`, `default`, `dotenv`, `invocation`, `new`, `nonempty`, `response`: Adapter support modules.
