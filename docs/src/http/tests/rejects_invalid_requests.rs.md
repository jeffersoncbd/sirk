## Summary
Tests that HTTP routes return expected responses for valid and invalid requests.

## Behavior
Checks successful health, OpenAPI, and Swagger responses and their content, then verifies that unknown routes return 404, unsupported methods return 405, and invalid POST bodies return 400.

## Imports
- `super::request::request`: Sends HTTP requests to the test handler.
- `axum::body::to_bytes`: Reads response bodies.
- `axum::http::header`: Provides the content-type header name.
