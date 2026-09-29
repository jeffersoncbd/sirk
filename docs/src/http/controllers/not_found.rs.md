## Summary
Returns a JSON 404 response for an unmatched request.

## Behavior
Builds a response with status `NOT_FOUND` and an error body containing `"Not found"`.

## Imports
- `JsonResponse`: Wraps the error response as JSON.
- `ErrorResponse`: Holds the not-found error message.
- `axum`: Provides the status code and response types.
