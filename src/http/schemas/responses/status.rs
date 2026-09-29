use super::SuccessStatus;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(in crate::http) struct StatusResponse {
    /// Fixed success indicator.
    pub(in crate::http) status: SuccessStatus,
}
