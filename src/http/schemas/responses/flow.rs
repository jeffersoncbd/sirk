use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(in crate::http) struct FlowResponse {
    /// Identifier required in X-Sirk-Flow-Id for the flow's later requests.
    #[serde(rename = "flowId")]
    pub(in crate::http) flow_id: String,
}
