## Summary
Runs an agent conversation, handling permitted tool requests until it returns a result or question.

## Behavior
Rejects unavailable user input, then saves the input and repeatedly invokes the configured adapter, recording prompts, responses, and usage. It executes permitted edit, write, delete, read, and tree requests and continues; unauthorized or invalid requests and other failures return errors.

## Imports
- `adapters`: Resolves the agent adapter.
- `RunRequest`: Supplies parameters for adapter invocation.
- `Block`, `Conversation`, `History`: Store and save conversation state.
- `Invocation`: Represents an adapter invocation.
- `tools`: Parses and executes read and tree requests.
