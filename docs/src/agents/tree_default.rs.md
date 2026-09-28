## Summary
Returns `true` as the default TREE tool permission for legacy agent snapshots.

## Behavior
Always returns `true`, preserving the implicit TREE permission used by snapshots created before `TREE_TOOL` existed. It performs no validation and has no side effects.

## Imports
- None.
