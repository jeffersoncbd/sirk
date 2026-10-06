## Summary
Defines the transport-independent agent execution module and exposes its execution API.

## Behavior
Declares internal modules and re-exports `AgentExecution`, `AgentOutcome`, and `run` for crate use.

## Imports
- `conversation`: Internal conversation module.
- `delete_request`: Internal deletion-request module.
- `edit_request`: Internal edit-request module.
- `execute`: Internal execution module.
- `outcome`: Provides `AgentExecution` and `AgentOutcome`.
- `prompt`: Internal prompt module.
- `run`: Provides the agent execution entry point.
- `write_request`: Internal write-request module.
