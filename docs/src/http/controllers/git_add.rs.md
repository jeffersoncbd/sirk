## Summary
`git_add` validates a directory request and adds that directory through the Git service.

## Behavior
It returns `400 Bad Request` for invalid UTF-8 or JSON. It runs the Git operation on a blocking task, returning a JSON success response or a `500 Internal Server Error` with the operation or task failure.

## Imports
- `DirectoryRequest`, `JsonResponse`: Request data and JSON responses.
- `axum`: Request body, status codes, and HTTP responses.
- `serde_json`: Builds and parses JSON.
