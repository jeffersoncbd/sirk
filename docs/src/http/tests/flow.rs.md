## Summary
Submits a flow creation request for a directory and returns its flow ID.

## Behavior
Sends a `POST` request to `/v1/flows` with the directory in the JSON body, asserts a `201` response, then reads and parses the response body and extracts `flowId` as a string. Body reading, JSON parsing, or field extraction failures panic.

## Imports
- `super::request::request`: Sends the HTTP request.
- `axum::body::to_bytes`: Reads the response body into bytes.
- `std::path::Path`: Provides the directory parameter type.
