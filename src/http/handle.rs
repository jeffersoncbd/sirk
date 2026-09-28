use super::types::{AgentRequest, DirectoryRequest, HttpResponse};
use serde_json::json;

pub(super) fn handle(method: &str, path: &str, body: &str) -> HttpResponse {
    if method == "GET" && path == "/health" {
        return HttpResponse {
            status: 200,
            body: json!({ "status": "ok" }).to_string(),
        };
    }
    if !matches!(path, "/v1/agent/run" | "/v1/git/status" | "/v1/git/add") {
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
    match path {
        "/v1/agent/run" => {
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
        "/v1/git/status" => {
            let request = match serde_json::from_str::<DirectoryRequest>(body) {
                Ok(request) => request,
                Err(_) => {
                    return HttpResponse {
                        status: 400,
                        body: json!({ "error": "Invalid request" }).to_string(),
                    };
                }
            };
            match crate::git_service::status(&request.directory) {
                Ok(paths) => HttpResponse {
                    status: 200,
                    body: json!({ "paths": paths }).to_string(),
                },
                Err(error) => HttpResponse {
                    status: 500,
                    body: json!({ "error": error }).to_string(),
                },
            }
        }
        "/v1/git/add" => {
            let request = match serde_json::from_str::<DirectoryRequest>(body) {
                Ok(request) => request,
                Err(_) => {
                    return HttpResponse {
                        status: 400,
                        body: json!({ "error": "Invalid request" }).to_string(),
                    };
                }
            };
            match crate::git_service::add(&request.directory) {
                Ok(()) => HttpResponse {
                    status: 200,
                    body: json!({ "status": "ok" }).to_string(),
                },
                Err(error) => HttpResponse {
                    status: 500,
                    body: json!({ "error": error }).to_string(),
                },
            }
        }
        _ => unreachable!("the route was validated above"),
    }
}
