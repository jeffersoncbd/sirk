## Summary
Declares the HTTP test modules and maps them to their test files.

## Behavior
Each module declaration loads its corresponding file under `tests/`.

## Imports
- `creates_flow_directory_layout`, `enforces_agent_delete_permissions`: Test modules.
- `flow`, `handles_agent_edits`, `handles_agent_requests`: Test modules.
- `handles_git_add_requests`, `handles_git_status_requests`: Test modules.
- `handles_tree_requests`, `logs_provider_token_usage`: Test modules.
- `openapi_is_current`, `persists_conversation_tool_context`: Test modules.
- `preserves_literal_template_input`, `rejects_invalid_requests`: Test modules.
- `rejects_unauthorized_agent_question`, `request`: Test modules.
- `resumes_agent_conversation`, `returns_agent_question`: Test modules.
