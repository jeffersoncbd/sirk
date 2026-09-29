## Summary
Groups the HTTP request and behavior test modules.

## Behavior
Declares eight test modules and maps each to its corresponding test file.

## Imports
- `enforces_agent_delete_permissions`: Tests agent deletion permissions.
- `handles_agent_edits`: Tests agent edits.
- `handles_agent_requests`: Tests agent requests.
- `handles_git_add_requests`: Tests Git add requests.
- `handles_git_status_requests`: Tests Git status requests.
- `preserves_literal_template_input`: Tests literal template input handling.
- `rejects_invalid_requests`: Tests invalid request rejection.
- `request`: Tests HTTP request behavior.
