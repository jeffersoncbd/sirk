## Summary
Runs the configured agent until it returns a result, asks a question, or encounters an error.

## Behavior
Rejects input when a question is already pending; otherwise records the input and repeatedly invokes the adapter, saving responses and usage. Executes requested tools only when permitted, continues after tool results, and returns either the final response or a validated question. Failures return errors.

## Imports
- `adapters`: Resolves the configured agent adapter.
- `RunRequest`: Supplies parameters for adapter invocation.
- `Block`, `Conversation`, `History`: Store and save conversation state.
- `Invocation`: Represents an adapter invocation.
- `tools`: Parses and executes read and tree requests.
