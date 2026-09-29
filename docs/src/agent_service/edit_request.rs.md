## Summary
Prepares and commits an edit request while preventing completed edits from being repeated.

## Behavior
Parses the request and rejects a supplied version. Returns a message if an identical edit is already in history. For writes, checks that the path is allowed; for other operations, reads the file and may add its version. Preparation failures return corrective messages. On success, records and saves the pending edit before committing it; parsing, serialization, save, and commit errors are returned as `Err`.

## Imports
- `Block`, `History`: History records and persistence.
- `Pending`, `Request`: Edit request preparation and commit.
- `read`: Checks write access and reads files.
- `serde_json`: Parses requests and serializes history.
