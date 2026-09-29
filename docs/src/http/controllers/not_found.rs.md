## Summary
Returns a JSON 404 response for an unmatched request.

## Behavior
Builds a response with status `NOT_FOUND` and a JSON body containing `"error": "Not found"`.

## Imports
- `JsonResponse`: Wraps the JSON error body.
- `axum`: Provides the status code and response types.
- `serde_json`: Constructs the JSON error body.
