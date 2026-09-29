## Summary
Returns a JSON response indicating that the requested method is not allowed.

## Behavior
Builds a response with HTTP status 405 and an error message, then converts it into an Axum response.

## Imports
- `ErrorResponse`: Defines the JSON error body.
- `JsonResponse`: Wraps the error body as JSON.
- `axum`: Provides the status code and response conversion.
