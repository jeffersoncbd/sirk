## Summary
`git_add` stages all changes within a validated directory and returns an HTTP response.

## Behavior
It returns `400 Bad Request` for invalid JSON or a missing, invalid, or mismatched flow ID. Otherwise, it runs Git add on a blocking task and returns `200 OK` on success or `500 Internal Server Error` if the operation or task fails.

## Imports
- `JsonResponse`: Wraps response bodies as JSON.
- `DirectoryRequest`: Supplies the directory to stage.
- `ErrorResponse`, `StatusResponse`, `SuccessStatus`: Define response bodies.
- `axum`: Extracts JSON and constructs HTTP responses.
- `tokio::task`: Runs Git add on a blocking thread.
