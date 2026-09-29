## Summary
This module re-exports HTTP response schema types.

## Behavior
It declares seven response submodules and re-exports their types within `crate::http`.

## Imports
- `agent_run`: Provides `AgentRunResponse`.
- `error`: Provides `ErrorResponse`.
- `git_status`: Provides `GitStatusResponse`.
- `health`: Provides `HealthResponse`.
- `status`: Provides `StatusResponse`.
- `success`: Provides `SuccessStatus`.
- `tree`: Provides `TreeResponse`.
