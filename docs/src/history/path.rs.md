## Summary
Builds the canonical log path for a history flow.

## Behavior
Canonicalizes the directory, converting any filesystem error to a `String`, then appends `history/<flow_id>/flow.log` and returns the path.

## Imports
- `super::History`: Provides the `History` type being extended.
- `std::path::Path`: Represents the input directory path.
