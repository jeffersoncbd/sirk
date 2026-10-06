## Summary
`AgentRunResponse` serializes an agent run’s conversation ID and optional result or question.

## Behavior
The response always includes `conversationId`; `result` and `ask` are omitted when absent. Unknown fields are rejected during deserialization.

## Imports
- `serde`: Provides serialization attributes and support.
- `utoipa`: Provides OpenAPI schema support.
