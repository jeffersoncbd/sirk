## Summary
Runs an agent conversation for a canonicalized directory and returns its result.

## Behavior
Loads the agent from `.agents`, checks that its adapter is available, creates conversation history, and executes the conversation. It records a usage summary before returning the conversation result; errors are returned as `String`.

## Imports
- `adapters`: Checks whether the agent’s adapter is available.
- `agents::Agent`: Loads the agent definition.
- `history::{History, Snapshot}`: Creates history and records usage.
- `std::path::Path`: Provides the directory input.
