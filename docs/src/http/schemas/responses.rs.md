## Summary
This module exposes HTTP response schema types within `crate::http`.

## Behavior
It declares six response schema submodules and re-exports their response types with visibility limited to the HTTP crate module.

## Imports
- `agent_run`: Provides `AgentRunResponse`.
- `error`: Provides `ErrorResponse`.
- `git_status`: Provides `GitStatusResponse`.
- `health`: Provides `HealthResponse`.
- `status`: Provides `StatusResponse`.
- `success`: Provides `SuccessStatus`.
