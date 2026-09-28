## Summary
Runs an agent conversation, processing responses and tools until a final response is received.

## Behavior
Records and saves the input; if a question is pending, saves it and returns an error. Resolves the adapter, executes calls, and saves each response. Handles edit, delete, read, or tree requests according to configured permissions; user-input requests and empty responses return errors. Execution and write errors are propagated.

## Imports
- `adapters`: Resolves the adapter and interprets responses.
- `RunRequest`: Holds parameters sent to the agent.
- `Block`, `History`: Store and persist the history.
- `Invocation`: Represents the agent call.
- `tools`: Identifies and runs tools.
