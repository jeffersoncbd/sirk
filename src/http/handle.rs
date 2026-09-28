use super::types::{AgentRequest, HttpResponse};
use serde_json::json;

pub(super) fn handle(method: &str, path: &str, body: &str) -> HttpResponse {
    if method == "GET" && path == "/health" {
        return HttpResponse {
            status: 200,
            body: json!({ "status": "ok" }).to_string(),
        };
    }
    if path != "/v1/agent/run" {
        return HttpResponse {
            status: 404,
            body: json!({ "error": "Not found" }).to_string(),
        };
    }
    if method != "POST" {
        return HttpResponse {
            status: 405,
            body: json!({ "error": "Method not allowed" }).to_string(),
        };
    }
    let request = match serde_json::from_str::<AgentRequest>(body) {
        Ok(request) => request,
        Err(_) => {
            return HttpResponse {
                status: 400,
                body: json!({ "error": "Invalid request" }).to_string(),
            };
        }
    };
    match crate::agent_service::run(&request.directory, request.agent, request.input) {
        Ok(result) => HttpResponse {
            status: 200,
            body: json!({ "result": result }).to_string(),
        },
        Err(error) => HttpResponse {
            status: 500,
            body: json!({ "error": error }).to_string(),
        },
    }
}
