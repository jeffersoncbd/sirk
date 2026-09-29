## Summary
`git_status` returns changed Git paths for a directory after validating its flow ID.

## Behavior
Invalid JSON or flow validation returns `400`; flow ID extraction may return an error response. The Git lookup runs on a blocking task. Success returns the paths; lookup or task failures return `500` with an error message.

## Imports
- `JsonResponse`: Wraps JSON response bodies
- `DirectoryRequest`: Supplies the requested directory
- `ErrorResponse`, `GitStatusResponse`: Define response bodies
- `axum`: Extracts JSON and builds HTTP responses
- `tokio::task`: Runs the Git lookup on a blocking task
