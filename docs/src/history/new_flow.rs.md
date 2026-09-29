## Summary
Creates a uniquely named history flow file and returns its flow ID.

## Behavior
Canonicalizes the directory, creates its `history` subdirectory, and repeatedly generates flow IDs until it can create a new flow file without overwriting an existing one. It syncs the file and history directory before returning the ID; filesystem and flow ID errors are returned as strings.

## Imports
- `super::History`: Builds the flow file path.
- `super::flow_id::flow_id`: Generates candidate flow IDs.
- `std::fs`: Creates the history directory.
- `std::fs::OpenOptions`: Creates a new flow file exclusively.
- `std::path::Path`: Accepts the directory path.
