## Summary
Prepares and commits an edit request while preventing completed edits from being repeated.

## Behavior
Parses the request and rejects a supplied version. Returns a message if an identical edit is already in history. For writes, checks that the path is allowed; for other operations, reads the file and may add its version. Preparation failures return corrective messages. On success, records the pending edit before committing it; parsing, serialization, and commit errors are returned as `Err`.

## Imports
- `Block`, `History`: Edit history and persistence.
- `Pending`, `Request`: Request preparation and commit.
- `read`: Checks path permissions and reads files.
- `serde_json`: Parses requests and serializes pending edits.
