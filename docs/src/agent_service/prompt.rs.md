## Summary
Builds the agent prompt from tool instructions and conversation history.

## Behavior
Starts with the agent instructions and adds available tools according to its permissions. Then includes history blocks with their roles, skipping a user entry when it follows an edit or delete request.

## Imports
- `crate::history::{Block, History}`: Types used to read and classify history.
