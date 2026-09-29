## Summary
Builds an HTTP request and dispatches it through the application router.

## Behavior
Sets the method, path, JSON content type, and body, then sends the request through `routes()` with `oneshot`. Panics if request construction or dispatch fails.

## Imports
- `routes`: Provides the application router.
- `axum`: Provides HTTP request, response, and body types.
- `tower::ServiceExt`: Provides the `oneshot` method.
