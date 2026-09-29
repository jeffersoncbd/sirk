use serde::Deserialize;
use std::path::PathBuf;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(in crate::http) struct AgentRunRequest {
    /// Server-visible execution directory.
    #[schema(value_type = String, examples("/workspace/project"))]
    pub(in crate::http) directory: PathBuf,
    /// Identifier of the `.agents/<agent>.md` definition to run.
    #[schema(examples("code-explainer"))]
    pub(in crate::http) agent: String,
    /// Literal text passed to the agent without template expansion.
    #[schema(examples("Explain src/lib.rs."))]
    pub(in crate::http) input: String,
}
