## Summary
Defines the HTTP transport module and re-exports its `serve` function.

## Behavior
Declares internal support modules for request handling, OpenAPI, Swagger, and types, then exposes `serve::serve` as the module’s public entry point.

## Imports
- `handle`: Internal request-handling support.
- `openapi`: OpenAPI support.
- `serve`: Provides the public `serve` function.
- `swagger`: Swagger support.
- `types`: Internal HTTP types.
