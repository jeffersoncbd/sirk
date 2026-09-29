## Summary
Returns a JSON response indicating that the requested method is not allowed.

## Behavior
Builds a response with HTTP status 405 and a JSON error message, then converts it into an Axum response.

## Imports
- `super::super::types::JsonResponse`: Wraps the JSON response body.
- `axum`: Provides the status code and response conversion.
- `serde_json::json`: Constructs the JSON error object.
