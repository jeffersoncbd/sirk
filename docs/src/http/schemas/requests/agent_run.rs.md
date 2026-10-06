## Summary
`AgentRunRequest` describes the input for running or continuing an agent conversation.

## Behavior
Deserialization rejects unknown fields. The request includes an execution directory, agent identifier, and literal input text; an optional `conversationId` selects an existing conversation, or can be omitted to start a new one.

## Imports
- `serde`: Deserializes the request and configures its JSON fields.
- `std::path`: Provides the execution directory path type.
- `utoipa`: Derives the OpenAPI schema.
