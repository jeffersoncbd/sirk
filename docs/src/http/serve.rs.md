## Summary
Runs an HTTP server that handles each incoming request on a separate thread.

## Behavior
Binds to the supplied address, returning bind errors as `String`, then reads each request body and passes it with the method and path to `handle`. If reading fails, it sends a 400 JSON error. It sends the resulting response with its status and content type, logging send failures to `stderr`.

## Imports
- `super::handle`: Processes the request method, path, and body.
- `super::types::HttpResponse`: Represents the response for an invalid body.
- `tiny_http`: Creates the server and builds and sends HTTP responses.
