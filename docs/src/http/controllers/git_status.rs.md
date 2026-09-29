## Summary
`git_status` returns changed Git paths for a requested directory.

## Behavior
Invalid JSON requests receive `400 Bad Request`. Valid requests run the Git status lookup on a blocking task; success returns the paths, while lookup or task failures return `500 Internal Server Error` with an error message.

## Imports
- `JsonResponse`: Wraps JSON response bodies
- `DirectoryRequest`: Provides the requested directory
- `ErrorResponse`, `GitStatusResponse`: Define response bodies
- `axum`: Provides JSON extraction and HTTP responses
- `tokio::task`: Runs the blocking Git lookup
