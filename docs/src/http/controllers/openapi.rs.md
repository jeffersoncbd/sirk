## Summary
Serves the binary’s OpenAPI specification as plain text.

## Behavior
Builds an HTTP response with the `TEXT` content type and `OPENAPI` as its body.

## Imports
- `super::super::content_type::TEXT`: Plain-text response content type.
- `super::super::openapi::OPENAPI`: OpenAPI document body.
- `axum::http::header`: Content-Type header name.
- `axum::response::{IntoResponse, Response}`: Builds and types the response.
