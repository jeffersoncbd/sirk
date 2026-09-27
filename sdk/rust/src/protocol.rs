use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(super) struct Request<'a> {
    pub jsonrpc: &'static str,
    pub id: u64,
    pub method: &'static str,
    pub params: AgentParams<'a>,
}

#[derive(Serialize)]
pub(super) struct AgentParams<'a> {
    pub agent: &'a str,
    pub input: &'a str,
}

#[derive(Deserialize)]
pub(super) struct Response {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<String>,
    pub error: Option<RpcError>,
}

#[derive(Deserialize)]
pub(super) struct RpcError {
    pub code: i64,
    pub message: String,
}
