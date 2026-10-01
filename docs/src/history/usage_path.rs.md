## Summary
Builds the canonical path to a flow’s usage log inside the history directory.

## Behavior
Canonicalizes the supplied directory and returns any filesystem error as a `String`; on success, appends `history/USAGE_{flow_id}.log`.

## Imports
- `std::path::Path`: Provides the directory path input.
