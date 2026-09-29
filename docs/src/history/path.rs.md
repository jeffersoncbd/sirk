## Summary
Builds the canonical log file path for a history flow.

## Behavior
Canonicalizes the directory, returning any filesystem error as a `String`, then appends `history/<flow_id>.log` and returns the resulting path.

## Imports
- `super::History`: Provides the `History` type being extended.
- `std::path::Path`: Represents the input directory path.
