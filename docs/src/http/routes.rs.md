## Summary
Builds the HTTP router and its OpenAPI document.

## Behavior
Creates the OpenAPI document and removes its license, registers the API endpoints, and sets handlers for unmatched paths and unsupported methods. Returns the router and document.

## Imports
- `super::controllers`: Supplies endpoint and fallback handlers.
- `super::spec::ApiDoc`: Defines the OpenAPI document.
- `axum::Router`: Provides the HTTP router type.
- `utoipa::OpenApi`: Generates the OpenAPI document.
- `utoipa_axum::router::OpenApiRouter`: Builds documented routes.
