## Summary
Returns the OpenAPI document as an HTTP response.

## Behavior
Sets the response content type to `TEXT_CONTENT_TYPE` and uses `OPENAPI` as the response body.

## Imports
- `super::super::openapi::OPENAPI`: OpenAPI document body.
- `super::super::types::TEXT_CONTENT_TYPE`: Response content type.
- `axum::http::header`: Content-Type header name.
- `axum::response::{IntoResponse, Response}`: Builds and types the response.
