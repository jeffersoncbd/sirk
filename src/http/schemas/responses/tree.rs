use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(in crate::http) struct TreeResponse {
    /// Sorted, unique paths relative to the requested directory.
    pub(in crate::http) paths: Vec<String>,
}
