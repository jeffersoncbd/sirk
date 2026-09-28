## Summary
Runs a conversation with an agent loaded for the specified directory.

## Behavior
Canonicalizes the directory, loads the agent from `.agents`, and checks whether its adapter is known. Creates the conversation history and delegates execution; path, agent, adapter, or conversation errors are returned as `String`.

## Imports
- `adapters`: Validates the agent adapter.
- `agents::Agent`: Loads the agent definition.
- `history::{History, Snapshot}`: Creates conversation history.
- `std::path::Path`: Represents the supplied directory.
