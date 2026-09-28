use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AgentRequest {
    pub directory: PathBuf,
    pub agent: String,
    pub input: String,
}

#[derive(Debug, PartialEq)]
pub(super) struct HttpResponse {
    pub status: u16,
    pub body: String,
}
