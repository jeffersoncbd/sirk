## Summary
This file contains only a test; it has no main function to document.

## Behavior
The test parses an agent with `ASK_TOOL: allow`, checks that `ask_tool` is enabled, and verifies serialization emits `ASK_TOOL: true`.

## Imports
- `super::super::Agent`: Parses the agent configuration.
- `serde_yaml`: Serializes the agent for the assertion.
