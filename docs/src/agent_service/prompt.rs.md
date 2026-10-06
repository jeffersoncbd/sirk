## Summary
Builds a prompt from agent instructions, enabled tools, and conversation history.

## Behavior
Adds instructions for enabled tools, always including READ, then appends labeled history blocks. Skips an input immediately following an assistant output that requests EDIT, WRITE, or DELETE.

## Imports
- `crate::history::{Block, History}`: Accesses and classifies conversation history.
