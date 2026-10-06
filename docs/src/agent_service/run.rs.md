## Summary
Runs an agent conversation and returns its outcome with the conversation ID.

## Behavior
Canonicalizes the directory, loads the agent and resolves its adapter, then opens or creates a conversation. Existing conversations must be awaiting user input. It runs the conversation, saves its updated status and blocks, and records usage; errors are returned as `String`.

## Imports
- `adapters`: Resolves the agent’s adapter.
- `agents::Agent`: Loads the agent definition.
- `history`: Creates conversation history and records status and usage.
- `std::path::Path`: Accepts the directory path.
