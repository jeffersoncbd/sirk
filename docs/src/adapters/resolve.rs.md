## Summary
Resolves a harness name to its adapter implementation.

## Behavior
Compares `name` with known identifiers and returns the corresponding adapter in `Some`. Returns `None` for unrecognized names.

## Imports
- `crate::harness::HarnessAdapter`: Type of the returned interface.
- `super`: Concrete adapters available for resolution.
