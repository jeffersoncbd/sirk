## Summary
Defines `History`, which holds conversation state, usage data, paths, and a lock.

## Behavior
`History` stores a snapshot and conversation blocks alongside usage calls and paths; its private file handle represents the lock. The module also re-exports related types and flow helpers.

## Imports
- `crate::interfaces`: Provides the `Block` and `Snapshot` types.
- `std`: Provides the lock file and path types.
- `usage_call::UsageCall`: Stores usage-call data.
- `valid_flow_id`: Re-exports the flow ID validator.
