## Summary
Creates an empty history for an existing flow in the snapshot.

## Behavior
Rejects invalid flow IDs, resolves the flow and resume paths, and returns an error if the flow file is missing. Acquires a lock and returns a `History` with empty blocks and resume calls, retaining the snapshot and lock.

## Imports
- `super`: Provides `History`, `Snapshot`, and flow ID validation.
