## Summary
`AgentRunResponse` serializes an agent run’s conversation ID and optional result or question.

## Behavior
Serialization omits `conversationId`, `result`, and `ask` when absent; `conversation_id` is renamed to `conversationId`. The type also derives OpenAPI schema support and rejects unknown fields during deserialization.

## Imports
- `serde`: Provides serialization derives and field attributes.
- `utoipa`: Provides OpenAPI schema support.
