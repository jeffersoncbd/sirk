## Summary
Builds and sends an HTTP request through the application routes, returning its response.

## Behavior
Creates a request from the supplied method, path, and body, then dispatches it through `routes()` using Tower’s `oneshot` service. It unwraps both request construction and dispatch results, so failures panic.

## Imports
- `super::super::routes::routes`: Provides the application router.
- `axum`: Provides HTTP request, response, and body types.
- `tower::ServiceExt`: Provides the `oneshot` dispatch method.
