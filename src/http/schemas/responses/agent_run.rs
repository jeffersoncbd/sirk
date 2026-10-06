use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(in crate::http) struct AgentRunResponse {
    /// Conversation identifier to send when answering an `ask` response.
    #[serde(rename = "conversationId")]
    pub(in crate::http) conversation_id: String,
    /// Final text returned by a completed agent run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::http) result: Option<String>,
    /// Question returned when an agent requests user input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(in crate::http) ask: Option<String>,
}
