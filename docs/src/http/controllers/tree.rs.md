## Summary
Lists the files beneath the requested directory and returns their relative paths as JSON.

## Behavior
Rejects invalid JSON with a 400 response. Runs the tree listing in a blocking task; on success, converts paths to UTF-8 strings and returns them, or returns a 500 error if listing, task execution, or path conversion fails.

## Imports
- `JsonResponse`: Wraps response bodies in JSON.
- `DirectoryRequest`: Provides the directory to list.
- `ErrorResponse`, `TreeResponse`: Define JSON response bodies.
- `axum`: Provides JSON extraction, HTTP status codes, and responses.
