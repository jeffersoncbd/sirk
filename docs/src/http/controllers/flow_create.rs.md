## Summary
Creates an empty flow transcript in the requested directory and returns its identifier.

## Behavior
Invalid JSON requests receive a 400 response. The handler creates the transcript in a blocking task; success returns the flow ID with status 201, while creation or task errors return status 500 with an error message.

## Imports
- `super::super`: Request, response, and JSON response types.
- `axum`: HTTP request extraction and response construction.
- `tokio`: Runs transcript creation in a blocking task.
- `crate::history`: Creates the flow transcript.
