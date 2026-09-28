## Summary
Exposes `run::run` as the entry point for agent execution.

## Behavior
Declares the service's internal modules and re-exports `run` with crate-level visibility; this file defines no function.

## Imports
- `conversation`: Internal conversation module.
- `delete_request`: Internal module for deletion requests.
- `edit_request`: Internal module for edit requests.
- `execute`: Internal execution module.
- `prompt`: Internal prompt module.
- `run::run`: Function re-exported for use within the crate.
