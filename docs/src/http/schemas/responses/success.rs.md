## Summary
`SuccessStatus` represents the successful status in HTTP responses.

## Behavior
Its `Ok` variant serializes as `"ok"` and is available within the HTTP module.

## Imports
- `serde::Serialize`: Enables serialization of the status.
- `utoipa::ToSchema`: Enables OpenAPI schema generation.
