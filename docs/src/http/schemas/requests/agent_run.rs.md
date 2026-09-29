## Summary
`AgentRunRequest` defines the HTTP request data for running an agent.

## Behavior
Deserialization rejects unknown fields. The request contains a server-visible execution directory, an agent definition identifier, and literal input text.

## Imports
- `serde`: Deserializes the request and rejects unknown fields.
- `std::path`: Provides the execution directory path type.
- `utoipa`: Derives the OpenAPI schema.
