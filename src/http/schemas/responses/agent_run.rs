use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(in crate::http) struct AgentRunResponse {
    /// Final text returned by the executed agent.
    pub(in crate::http) result: String,
}
