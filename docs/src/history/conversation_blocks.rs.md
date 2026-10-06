## Summary
`Conversation::blocks` converts conversation messages into their corresponding blocks.

## Behavior
It iterates over messages in order, cloning each message’s content into an input, output, or tool-specific block, then collects the blocks into a vector.

## Imports
- `Block`, `Conversation`, `ConversationMessage`, `ConversationTool`: Types used to map messages to blocks.
