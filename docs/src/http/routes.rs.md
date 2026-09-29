## Summary
`routes` builds the HTTP router and connects paths to their controllers.

## Behavior
Registers GET routes for health, OpenAPI, and Swagger, plus POST routes for agent execution and Git status/add operations. Unmatched paths use the not-found handler, and unsupported methods use the method-not-allowed handler.

## Imports
- `super::controllers`: Provides endpoint and fallback handlers.
- `axum`: Provides the router and route registration types.
