## Summary
`handle` serves health and API documentation endpoints and dispatches agent and Git requests.

## Behavior
Returns the health, OpenAPI, or Swagger response for matching GET routes. Unknown paths return `404`, and known paths with unsupported methods return `405`. POST requests deserialize the appropriate JSON body; invalid bodies return `400`, service errors return `500`, and successful operations return `200`.

## Imports
- `super::openapi`: Supplies the OpenAPI document.
- `super::swagger`: Supplies the Swagger UI page.
- `super::types`: Provides request, response, and content-type definitions.
- `serde_json`: Parses requests and builds JSON responses.
- `crate::agent_service`: Runs agent requests.
- `crate::git_service`: Reads Git status and stages changes.
