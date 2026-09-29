## Summary
Builds the agent prompt from its instructions, enabled tools, and conversation history.

## Behavior
Adds tool instructions according to the agent’s settings, then appends labeled history blocks. Skips a user input that immediately follows an EDIT, WRITE, or DELETE request.

## Imports
- `crate::history::{Block, History}`: Reads and classifies conversation history.
