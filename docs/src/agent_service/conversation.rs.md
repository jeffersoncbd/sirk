## Summary
Runs an agent conversation, handling tool requests until it receives a final response.

## Behavior
Saves the input and each response to history, then repeatedly invokes the configured adapter. It handles edit, write, delete, read, and tree requests subject to the agent’s permissions; pending or requested user input and empty responses return errors. Adapter, tool, and history errors are propagated.

## Imports
- `adapters`: Resolves the adapter and parses its responses.
- `RunRequest`: Supplies parameters for each agent invocation.
- `Block`, `History`: Store and persist conversation history.
- `Invocation`: Represents an agent call.
- `tools`: Parses and executes agent tool requests.
