## Summary
Defines the health endpoint’s success response.

## Behavior
`HealthResponse` contains a `status` field with the `SuccessStatus` type. Serialization rejects unknown fields.

## Imports
- `super::SuccessStatus`: Type of the response status field.
- `serde::Serialize`: Enables response serialization.
- `utoipa::ToSchema`: Enables OpenAPI schema generation.
