## Summary
Runs an agent conversation and returns its outcome and optional conversation ID.

## Behavior
Canonicalizes the directory, loads the agent, validates its adapter, and opens a conversation if an ID is provided. Resumed conversations must be awaiting user input. It runs the conversation, saves updated blocks and status, and records usage; errors are returned as strings.

## Imports
- `adapters`: Resolves the agent’s adapter.
- `agents::Agent`: Loads the agent definition.
- `history`: Opens conversations and records status and usage.
- `std::path::Path`: Accepts the directory path.
