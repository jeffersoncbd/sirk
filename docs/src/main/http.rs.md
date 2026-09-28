## Summary
Starts the HTTP server at the supplied address or `127.0.0.1:8080`.

## Behavior
Uses the supplied address when available; otherwise, applies the default. Returns the result of `sirk::http::serve`, including any errors.

## Imports
- `sirk::http`: Starts the HTTP server.
