## Summary
Lists files beneath a validated directory and returns their paths as JSON.

## Behavior
Rejects invalid JSON or flow IDs with a 400 response. Runs the file listing in a blocking task; returns UTF-8 paths on success, or a 500 response if listing, task execution, or path conversion fails.

## Imports
- `JsonResponse`: Wraps response bodies in JSON.
- `DirectoryRequest`: Supplies the directory to list.
- `ErrorResponse`, `TreeResponse`: Define JSON response bodies.
- `axum`: Provides request extraction and HTTP response types.
