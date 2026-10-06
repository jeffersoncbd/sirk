## Summary
Builds a canonical path to a flow’s usage log.

## Behavior
Canonicalizes `directory`, converting any filesystem error to a `String`, then appends `history/{flow_id}/usage.log` and returns the path.

## Imports
- `super::History`: Defines the type whose implementation contains this method.
- `std::path::Path`: Provides the directory path input.
