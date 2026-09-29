## Summary
Verifies that HTTP routes return the expected responses for valid and invalid requests.

## Behavior
Checks the health, OpenAPI, and Swagger responses, including their content and content types. It also confirms that unknown routes return 404, unsupported methods return 405, and malformed POST requests return 400.

## Imports
- `super::request::request`: Sends requests to the test handler.
- `axum::body::to_bytes`: Reads response bodies.
- `axum::http::header`: Provides the content-type header name.
