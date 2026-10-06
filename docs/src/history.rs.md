## Summary
Defines `History`, which stores conversation state, usage data, file paths, and a lock.

## Behavior
The struct holds a snapshot, conversation blocks, usage calls, and paths; its private file handle keeps the lock.

## Imports
- `crate::interfaces`: Supplies the `Block` and `Snapshot` types.
- `std`: Supplies the file handle and path type.
- `usage_call::UsageCall`: Supplies usage-call data.
