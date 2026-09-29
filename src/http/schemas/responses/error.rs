use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(in crate::http) struct ErrorResponse {
    /// Human-readable failure message.
    pub(in crate::http) error: String,
}
