## Summary
`FlowResponse` serializes a flow identifier for HTTP responses.

## Behavior
The response contains a `flowId` string, documented for use in the `X-Sirk-Flow-Id` header on later requests. Unknown fields are denied during deserialization.

## Imports
- `serde`: Provides serialization and field renaming.
- `utoipa`: Generates the response schema.
