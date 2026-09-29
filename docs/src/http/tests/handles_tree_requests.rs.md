## Summary
`handles_tree_requests` verifies that `POST /v1/tree` returns repository paths while excluding files named in `.treeignore`.

## Behavior
The test creates a temporary Git repository with one visible file and one ignored file, sends a tree request, and checks for HTTP 200 and the expected paths. It removes the temporary directory afterward.

## Imports
- `super::{flow, request}`: Creates a flow and sends the HTTP request.
- `crate::services::{BashService, Invocation}`: Initializes the temporary Git repository.
- `axum::body::to_bytes`: Reads the response body.
- `std::{fs, io, time}`: Creates files, captures command output, and names the temporary directory.
