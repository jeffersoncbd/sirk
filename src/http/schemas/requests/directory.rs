use serde::Deserialize;
use std::path::PathBuf;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub(in crate::http) struct DirectoryRequest {
    /// Server-visible directory within a Git working tree.
    #[schema(value_type = String, examples("/workspace/project"))]
    pub(in crate::http) directory: PathBuf,
}
