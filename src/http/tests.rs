#[path = "tests/enforces_agent_delete_permissions.rs"]
mod enforces_agent_delete_permissions;
#[path = "tests/handles_agent_edits.rs"]
mod handles_agent_edits;
#[path = "tests/handles_agent_requests.rs"]
mod handles_agent_requests;
#[path = "tests/handles_git_add_requests.rs"]
mod handles_git_add_requests;
#[path = "tests/handles_git_status_requests.rs"]
mod handles_git_status_requests;
#[path = "tests/preserves_literal_template_input.rs"]
mod preserves_literal_template_input;

#[path = "tests/rejects_invalid_requests.rs"]
mod rejects_invalid_requests;
#[path = "tests/request.rs"]
mod request;
