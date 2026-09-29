use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub(in crate::http) enum SuccessStatus {
    Ok,
}
