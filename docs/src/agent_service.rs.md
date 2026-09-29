## Summary
Provides the transport-independent agent execution module and re-exports its entry point.

## Behavior
Declares the service’s internal modules and makes `run::run` available within the crate.

## Imports
- `conversation`: Internal conversation module.
- `delete_request`: Internal deletion-request module.
- `edit_request`: Internal edit-request module.
- `execute`: Internal execution module.
- `prompt`: Internal prompt module.
- `run::run`: Re-exported agent execution entry point.
- `write_request`: Internal write-request module.
