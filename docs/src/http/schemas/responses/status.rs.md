## Summary
Defines the HTTP status response with a fixed success indicator.

## Behavior
`StatusResponse` serializes a `SuccessStatus` field and derives an OpenAPI schema; unknown fields are denied during deserialization.

## Imports
- `super::SuccessStatus`: Type of the response status field.
- `serde::Serialize`: Enables response serialization.
- `utoipa::ToSchema`: Enables OpenAPI schema generation.
