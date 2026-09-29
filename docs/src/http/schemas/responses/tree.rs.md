## Summary
`TreeResponse` represents the paths returned by a tree response.

## Behavior
The response contains sorted, unique paths relative to the requested directory. Serialization rejects unknown fields.

## Imports
- `serde::Serialize`: Enables serialization of the response.
- `utoipa::ToSchema`: Generates the OpenAPI schema.
