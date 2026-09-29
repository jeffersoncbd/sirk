## Summary
Checks that HTTP routes return the expected responses for valid and invalid requests.

## Behavior
Sends requests to health, OpenAPI, Swagger, and API routes. Verifies success responses and content, plus 404 for an unknown route, 405 for unsupported methods, and 400 for malformed POST requests.

## Imports
- `super::request::request`: Sends requests to the test handler.
- `axum::body::to_bytes`: Reads response bodies.
- `axum::http::header`: Provides the content-type header name.
