use axum::{
    Json,
    http::header,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::Value;
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

pub(super) struct JsonResponse(pub(super) Value);

impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response {
        ([(header::CONTENT_TYPE, JSON_CONTENT_TYPE)], Json(self.0)).into_response()
    }
}
