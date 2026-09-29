## Summary
Defines history record types for snapshots and recorded content blocks.

## Behavior
`Snapshot` stores a directory and agent and supports serialization and deserialization while rejecting unknown fields. `Block` represents recorded asks, inputs, outputs, trees, reads, edits, writes, and deletes, each with associated text.

## Imports
- `crate::agents::Agent`: Agent stored in a snapshot.
- `serde`: Serialization and deserialization derives.
- `std::path::PathBuf`: Snapshot directory path.
