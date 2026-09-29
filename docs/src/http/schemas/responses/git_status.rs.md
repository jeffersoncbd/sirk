## Summary
`GitStatusResponse` represents Git status paths in an HTTP response.

## Behavior
The response contains `paths`, documented as sorted, unique paths relative to the requested directory.

## Imports
- `serde`: Provides serialization support.
- `utoipa`: Provides OpenAPI schema support.
