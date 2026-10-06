## Summary
Runs an agent conversation until it returns a result or asks a question.

## Behavior
Rejects unavailable user input; otherwise records and saves it, then repeatedly invokes the configured adapter and records its responses and usage. It executes permitted edit, write, delete, read, and tree requests before continuing. Unauthorized or invalid requests and other failures return errors.

## Imports
- `adapters`: Resolves the configured agent adapter.
- `RunRequest`: Supplies parameters for adapter invocation.
- `Block`, `Conversation`, `History`: Store and save conversation state.
- `Invocation`: Represents an adapter invocation.
- `tools`: Parses and executes read and tree requests.
