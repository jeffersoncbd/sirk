## Summary
Runs an agent conversation for the canonicalized directory and returns its result.

## Behavior
Loads the named agent from `.agents`, verifies its adapter, creates history for the flow, and delegates to the conversation executor. It records a resume afterward; failures are returned as `String` errors.

## Imports
- `adapters`: Verifies that the agent’s adapter is available.
- `agents::Agent`: Loads the agent definition.
- `history::{History, Snapshot}`: Creates and records conversation history.
- `std::path::Path`: Provides the directory input.
