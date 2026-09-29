## Summary
`AgentRunResponse` represents the final text returned by an executed agent.

## Behavior
Its `result` field is serialized and included in the OpenAPI schema; unknown fields are rejected during deserialization.

## Imports
- `serde`: Provides serialization support.
- `utoipa`: Provides OpenAPI schema support.
