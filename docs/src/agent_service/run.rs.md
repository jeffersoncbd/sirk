## Summary
Runs an agent conversation for the specified directory and input.

## Behavior
Canonicalizes the directory, loads the named agent from `.agents`, verifies its adapter, creates conversation history for the flow, and delegates execution. Errors are returned as `String`.

## Imports
- `adapters`: Checks whether the agent’s adapter is available.
- `agents::Agent`: Loads the agent definition.
- `history::{History, Snapshot}`: Creates conversation history.
- `std::path::Path`: Represents the supplied directory.
