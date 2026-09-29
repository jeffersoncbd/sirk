## Summary
Resolves a harness name to its adapter implementation.

## Behavior
Matches `name` against known identifiers and returns the corresponding default adapter in `Some`; returns `None` for unrecognized names.

## Imports
- `crate::harness::HarnessAdapter`: Trait for the returned adapter.
- `super`: Concrete adapters returned for recognized names.
