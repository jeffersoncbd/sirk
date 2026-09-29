use serde::Deserialize;
use std::path::PathBuf;

pub(super) const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";
pub(super) const JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";
pub(super) const TEXT_CONTENT_TYPE: &str = "text/plain; charset=utf-8";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AgentRequest {
    pub directory: PathBuf,
    pub agent: String,
    pub input: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DirectoryRequest {
    pub directory: PathBuf,
}

#[derive(Debug, PartialEq)]
pub(super) struct HttpResponse {
    pub status: u16,
    pub body: String,
    pub content_type: &'static str,
}
