## Summary
`handles_tree_requests` tests that `POST /v1/tree` returns visible Git paths while excluding files listed in `.treeignore`.

## Behavior
The test creates a temporary Git repository with visible and ignored files, sends a tree request, and asserts a successful response containing only `.treeignore` and `visible.rs`. It removes the temporary directory afterward.

## Imports
- `super::request::request`: Sends the HTTP request.
- `crate::services::{BashService, Invocation}`: Initializes the test Git repository.
- `axum::body::to_bytes`: Reads the response body.
- `std::{fs, io, time}`: Creates files, captures command output, and names the temporary directory.
