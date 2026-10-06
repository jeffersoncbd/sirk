## Summary
Replaces the conversation’s messages with messages mapped from the provided blocks.

## Behavior
Skips `Ask` blocks and converts each other block into a cloned user, assistant, or named tool message, preserving block order. Replaces the existing message list; the function has no error return.

## Imports
- `Block`: Block variants used to build conversation messages.
- `ConversationMessage`: Message types stored in the conversation.
- `ConversationTool`: Tool names assigned to tool messages.
