## Summary
`git_add` stages all changes under the requested directory and returns an HTTP response.

## Behavior
It returns `400 Bad Request` if the JSON request is invalid. Otherwise, it runs the Git add operation on a blocking task and returns `200 OK` on success or `500 Internal Server Error` with an error message if the operation or task fails.

## Imports
- `JsonResponse`: Wraps response bodies as JSON.
- `DirectoryRequest`: Provides the directory to stage.
- `ErrorResponse`, `StatusResponse`, `SuccessStatus`: Define response bodies.
- `axum`: Extracts JSON and builds HTTP responses.
- `tokio::task`: Runs the Git operation on a blocking thread.
