## Summary
Creates an empty history for a valid flow associated with the snapshot.

## Behavior
Validates the flow ID, resolves its path, and returns an error if the ID is invalid or the flow file is missing. It acquires a lock and returns a `History` with empty blocks, retaining the snapshot and lock.

## Imports
- `super`: Provides `History`, `Snapshot`, and flow ID validation.
