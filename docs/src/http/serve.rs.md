## Summary
Starts an HTTP server that processes requests in separate threads.

## Behavior
Binds the server to the supplied address and returns initialization errors as `String`. For each request, reads the body and calls `handle`; a read failure produces a 400 response. Sends the response as JSON and logs send failures to `stderr`.

## Imports
- `super::handle`: Processes the request method, path, and body.
- `super::types`: Provides the HTTP response structure.
- `tiny_http`: Creates the server and builds and sends HTTP responses.
