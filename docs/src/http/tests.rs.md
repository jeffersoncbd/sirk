## Summary
Declares the HTTP test modules.

## Behavior
Maps each test module to its file under `tests/`.

## Imports
- `enforces_agent_delete_permissions`: Tests agent deletion permissions.
- `handles_agent_edits`: Tests agent edits.
- `handles_agent_requests`: Tests agent requests.
- `handles_git_add_requests`: Tests Git add requests.
- `handles_git_status_requests`: Tests Git status requests.
- `handles_tree_requests`: Tests tree requests.
- `openapi_is_current`: Checks generated OpenAPI.
- `preserves_literal_template_input`: Tests literal template input.
- `rejects_invalid_requests`: Tests invalid request rejection.
- `request`: Tests HTTP requests.
