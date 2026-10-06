## Summary
This file contains a test that checks whether an agent enables delete permissions when explicitly allowed.

## Behavior
The test parses agent configuration with both delete options set to `allow`, then asserts that both permission flags are enabled.

## Imports
- `super::super::Agent`: Parses the agent configuration.
