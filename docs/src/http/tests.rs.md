## Summary
Groups the HTTP test modules.

## Behavior
Declares nine test modules and maps each to its corresponding test file.

## Imports
- `enforces_agent_delete_permissions`: Tests agent deletion permissions.
- `handles_agent_edits`: Tests agent edits.
- `handles_agent_requests`: Tests agent requests.
- `handles_git_add_requests`: Tests Git add requests.
- `handles_git_status_requests`: Tests Git status requests.
- `openapi_is_current`: Checks that generated OpenAPI is current.
- `preserves_literal_template_input`: Tests literal template input handling.
- `rejects_invalid_requests`: Tests invalid request rejection.
- `request`: Tests HTTP request behavior.
