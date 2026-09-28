use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize)]
pub(super) struct AgentRequest<'a> {
    pub directory: &'a Path,
    pub agent: &'a str,
    pub input: &'a str,
}

#[derive(Deserialize)]
pub(super) struct AgentResponse {
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub(super) struct HealthResponse {
    pub status: String,
}
