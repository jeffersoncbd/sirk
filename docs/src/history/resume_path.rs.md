## Summary
Builds the canonical path to a flow’s resume log.

## Behavior
Canonicalizes the directory, returning any filesystem error as a `String`, then appends `history/RESUME_<flow_id>.log`.

## Imports
- `std::path::Path`: Provides the directory path input.
