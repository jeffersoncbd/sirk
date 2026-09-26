//! Parsing and recovery helpers for agent-requested tools.

#[path = "external/completed_delete.rs"]
mod completed_delete;
#[path = "external/completed_edit.rs"]
mod completed_edit;
#[path = "external/delete_pending.rs"]
mod delete_pending;
#[path = "external/delete_request.rs"]
mod delete_request;
#[path = "external/edit_pending.rs"]
mod edit_pending;
#[path = "external/edit_request.rs"]
mod edit_request;
#[path = "external/prepare_edit.rs"]
mod prepare_edit;
#[path = "external/tool_result.rs"]
mod tool_result;
#[path = "external/user_tool_request.rs"]
mod user_tool_request;
#[path = "external/validate_user_tool_result.rs"]
mod validate_user_tool_result;

use serde::{Deserialize, Serialize};

pub(super) use completed_delete::completed_external_delete;
pub(super) use completed_edit::completed_external_edit;
pub(super) use delete_pending::external_delete_pending;
pub(super) use delete_request::external_delete_request;
pub(super) use edit_pending::external_edit_pending;
pub(super) use edit_request::external_edit_request;
pub(super) use prepare_edit::prepare_external_edit;
pub(super) use tool_result::tool_result;
pub(super) use user_tool_request::user_tool_request;
pub(super) use validate_user_tool_result::validate_user_tool_result;

pub(crate) const DUPLICATE_EDIT_RESULT: &str = "No changes applied: this EDIT request was already completed. Do not repeat it; provide a final response or a different edit.";
pub(crate) const EDIT_FAILURE_PREFIX: &str = "EDIT failed: ";
pub(super) const DUPLICATE_DELETE_RESULT: &str = "No changes applied: this DELETE request was already completed. Do not repeat it; provide a final response or a different request.";
pub(super) const DELETE_FAILURE_PREFIX: &str = "DELETE failed: ";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct ExternalDeleteRequest {
    pub(super) path: String,
    #[serde(default)]
    pub(super) force: bool,
}
