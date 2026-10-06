## Summary
This file contains no production function to document; it only defines a test.

## Behavior
The test checks that an agent requesting a file read without permission receives an HTTP 500 response with the expected error.

## Imports
- `super::{flow, request}`: Build the flow and send the test request.
- `axum::body`: Read the response body.
- `serde_json`: Parse the response and build the request payload.
- `std`: Create and clean up test files and directories.
