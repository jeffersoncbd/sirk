## Summary
Validates a flow ID and confirms that its corresponding file exists.

## Behavior
Rejects invalid IDs, resolves the flow path (propagating any error), and returns `Ok(())` if it is a file; otherwise, returns an unknown-flow error.

## Imports
- `super::{History, valid_flow_id}`: Validates IDs and resolves flow paths.
- `std::path::Path`: Provides the directory path type.
