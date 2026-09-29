## Summary
Runs the HTTP server at the supplied address on a multi-threaded Tokio runtime.

## Behavior
Builds the runtime, creates the router, and binds a TCP listener to `address`. It logs the listening URL, then serves the router; runtime, bind, and server errors are returned as strings.

## Imports
- `super::routes::routes`: Provides the HTTP router.
