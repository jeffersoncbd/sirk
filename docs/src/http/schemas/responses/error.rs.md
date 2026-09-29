## Summary
`ErrorResponse` represents an HTTP error response with a human-readable message.

## Behavior
The response contains an `error` string, serializes with Serde, and exposes an OpenAPI schema. Unknown fields are rejected during deserialization.

## Imports
- `serde`: Provides response serialization.
- `utoipa`: Provides OpenAPI schema support.
