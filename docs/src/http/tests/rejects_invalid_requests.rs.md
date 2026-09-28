## Summary
Defines no main function; contains only a test of HTTP responses.

## Behavior
The test checks that `/health` returns 200, an unknown route returns 404, unsupported GET requests return 405, and invalid bodies return 400.

## Imports
- `super::super::handle::handle`: Runs the HTTP requests checked by the test.
