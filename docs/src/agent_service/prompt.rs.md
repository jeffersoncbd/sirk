## Summary
Builds an agent prompt from its instructions, enabled tools, and conversation history.

## Behavior
Appends guidance for each enabled tool, then adds labeled history blocks. Skips a user input immediately following an assistant request to edit, write, or delete a file.

## Imports
- `crate::history::{Block, History}`: Reads and classifies conversation blocks.
