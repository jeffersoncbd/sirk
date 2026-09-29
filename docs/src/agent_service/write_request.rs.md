## Summary
`write_request` converts a JSON payload into a write operation and delegates it to `edit_request`.

## Behavior
It parses the payload as JSON, returns an error if parsing fails or the value is not an object, then sets the `operation` field to `"write"`. It serializes the updated request and passes it with the history to `edit_request`, returning that function’s result.

## Imports
- `crate::history::History`: Mutable history passed to the delegated request handler.
