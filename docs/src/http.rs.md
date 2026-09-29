## Summary
Exposes the HTTP server entry point and OpenAPI document.

## Behavior
Declares the HTTP support modules and re-exports `serve::serve` and `spec::document` for use outside this module.

## Imports
- `serve`: Provides the public HTTP server function.
- `spec`: Provides the generated OpenAPI document.
