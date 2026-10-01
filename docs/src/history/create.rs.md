## Summary
Creates an empty history for a valid, existing flow.

## Behavior
Validates the flow ID, resolves the flow and usage paths, and returns an error if the flow file is missing. Acquires a lock and returns a `History` with empty blocks and usage calls, retaining the snapshot and lock.

## Imports
- `super`: Provides `History`, `Snapshot`, and flow ID validation.
