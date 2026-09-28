## Summary
`handle` routes health checks and HTTP requests to agent execution and Git operations.

## Behavior
Returns `200` for `GET /health`, rejects unknown routes with `404`, and rejects invalid methods with `405`. For POST routes, deserializes the body for the operation: format errors return `400`, service errors return `500`, and successful results return `200`.

## Imports
- `super::types`: Request and HTTP-response types.
- `serde_json`: Creates JSON response bodies.
