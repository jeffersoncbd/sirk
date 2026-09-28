## Summary
Prepares and applies an edit request while preventing a completed edit from being repeated.

## Behavior
Parses the payload as `Request` and rejects versions supplied by the requester. If the same edit already appears in history, returns a message without applying it. Otherwise, reads the file, calculates its version, prepares the edit, records and saves the request in history before confirming the change. Read or preparation errors become messages for correcting the request; parsing, serialization, save, or confirmation errors are propagated as `Err`.

## Imports
- `Block`, `History`: History and request recording.
- `Pending`, `Request`: Edit representation and preparation.
- `read`: Reads the file before editing.
- `serde_json`: JSON parsing and serialization.
