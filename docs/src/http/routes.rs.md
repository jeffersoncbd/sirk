## Summary
`routes` builds the HTTP router and its OpenAPI document.

## Behavior
Creates the API document with its license omitted, registers the health, OpenAPI, Swagger, agent, and Git endpoints, then assigns fallbacks for unmatched paths and unsupported methods. Returns the router and document.

## Imports
- `super::controllers`: Provides endpoint and fallback handlers.
- `super::spec::ApiDoc`: Supplies the OpenAPI document definition.
- `axum::Router`: Provides the HTTP router type.
- `utoipa::OpenApi`: Provides the `openapi` method for `ApiDoc`.
- `utoipa_axum::router::OpenApiRouter`: Builds routes with OpenAPI metadata.
