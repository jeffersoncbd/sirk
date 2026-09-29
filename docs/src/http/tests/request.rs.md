## Summary
Builds a JSON HTTP request, optionally adds a flow ID header, and dispatches it through the application router.

## Behavior
Sets the method, path, content type, and body, then adds `X-Sirk-Flow-Id` when provided. Sends the request through `routes()` with `oneshot` and panics if construction or dispatch fails.

## Imports
- `routes`: Provides the application router.
- `axum`: Provides HTTP request, response, and body types.
- `tower::ServiceExt`: Provides the `oneshot` method.
