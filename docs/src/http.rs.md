## Summary
Exposes the HTTP server entry point and OpenAPI document.

## Behavior
Declares HTTP support modules and re-exports `serve::serve` and `spec::document` as `openapi_document`.

## Imports
- `serve`: Provides the public HTTP server function.
- `spec`: Provides the OpenAPI document.
