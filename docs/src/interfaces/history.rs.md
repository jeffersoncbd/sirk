## Summary
Defines the data types used to represent history records.

## Behavior
`Snapshot` holds the directory and agent, supports serialization, and rejects unknown fields. `Block` enumerates recorded content types, each associated with a string.

## Imports
- `crate::agents::Agent`: Agent type stored in the snapshot.
- `serde`: Derives serialization and deserialization for `Snapshot`.
- `std::path::PathBuf`: Represents the snapshot directory.
