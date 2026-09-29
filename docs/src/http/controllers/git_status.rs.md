## Summary
`git_status` returns the Git status paths for a requested directory.

## Behavior
It rejects non-UTF-8 bodies and invalid JSON with `400 Bad Request`. It runs the status lookup on a blocking task, returning paths on success or a `500 Internal Server Error` with an error message if the lookup or task fails.

## Imports
- `DirectoryRequest`, `JsonResponse`: Request and JSON response types
- `axum`: HTTP body, status codes, and response conversion
- `serde_json::json`: Builds JSON response bodies
