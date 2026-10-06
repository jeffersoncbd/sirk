## Summary
This test verifies that malformed agent definitions and unsafe IDs are rejected.

## Behavior
It checks that `Agent::parse` returns an error for invalid definition strings and `Agent::load` returns an error for empty, traversal, absolute, or filename-like IDs.

## Imports
- `Agent`: Parses definitions and loads agents.
- `std::path::Path`: Creates the agent directory path.
