## Summary
Validates a file-deletion request, prevents repeats, and performs authorized forced deletion.

## Behavior
Parses JSON with only `path` and optional `force` fields. Rejects unauthorized forced requests and reports requests already completed; otherwise records the payload in history. Non-forced requests return an error because user input is unavailable. Forced requests attempt deletion and return an empty success string or a failure message.

## Imports
- `crate::history::{Block, History}`: Reads settings and tracks request history.
- `serde::Deserialize`: Deserializes the request payload.
