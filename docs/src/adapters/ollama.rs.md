## Summary
Converts a request into an Ollama CLI invocation.

## Behavior
Requires a model and rejects requests with event streaming; otherwise, builds an `ollama run` invocation with the model, prompt, and working directory, without additional environment variables.

## Imports
- `crate::harness`: Request, invocation, and adapter-error types.
- `default`: Default adapter implementation.
- `new`: Adapter constructor.
