## Summary
Tests that the HTTP handler returns the expected responses for documented routes and invalid requests.

## Behavior
Checks successful health, OpenAPI, and Swagger responses, including their content; verifies unknown routes return 404, unsupported methods return 405, and invalid POST bodies return 400.

## Imports
- `super::super::handle::handle`: Handles the HTTP requests under test.
