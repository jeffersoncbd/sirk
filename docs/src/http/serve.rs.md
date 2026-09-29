## Summary
Runs the HTTP server at the supplied address using a multi-threaded Tokio runtime.

## Behavior
Builds the runtime, binds a TCP listener, logs the listening address, and serves the routes. Runtime, bind, or server errors are returned as strings.

## Imports
- `super::routes::routes`: Provides the HTTP router.
