## Summary
The file contains a test that checks whether an `Agent` enables the tree tool when explicitly allowed.

## Behavior
The test parses an agent definition with `TREE_TOOL: allow`, then asserts the flag is enabled and serialized output contains `TREE_TOOL: true`.

## Imports
- `super::super::Agent`: Parses the agent definition.
