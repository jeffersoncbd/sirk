## Summary
This module makes HTTP response schema types available within `crate::http`.

## Behavior
It declares eight response submodules and re-exports their types with visibility limited to `crate::http`.

## Imports
- `agent_run`: Provides `AgentRunResponse`.
- `error`: Provides `ErrorResponse`.
- `flow`: Provides `FlowResponse`.
- `git_status`: Provides `GitStatusResponse`.
- `health`: Provides `HealthResponse`.
- `status`: Provides `StatusResponse`.
- `success`: Provides `SuccessStatus`.
- `tree`: Provides `TreeResponse`.
