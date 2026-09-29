use super::{
    openapi::OPENAPI,
    swagger::SWAGGER_UI,
    types::{
        AgentRequest, DirectoryRequest, HTML_CONTENT_TYPE, HttpResponse, JSON_CONTENT_TYPE,
        TEXT_CONTENT_TYPE,
    },
};
use serde_json::json;

pub(super) fn handle(method: &str, path: &str, body: &str) -> HttpResponse {
    if method == "GET" && path == "/health" {
        return HttpResponse {
            status: 200,
            body: json!({ "status": "ok" }).to_string(),
            content_type: JSON_CONTENT_TYPE,
        };
    }
    if method == "GET" && path == "/openapi.yaml" {
        return HttpResponse {
            status: 200,
            body: OPENAPI.to_owned(),
            content_type: TEXT_CONTENT_TYPE,
        };
    }
    if method == "GET" && path == "/swagger" {
        return HttpResponse {
            status: 200,
            body: SWAGGER_UI.to_owned(),
            content_type: HTML_CONTENT_TYPE,
        };
    }
    if !matches!(
        path,
        "/health"
            | "/openapi.yaml"
            | "/swagger"
            | "/v1/agent/run"
            | "/v1/git/status"
            | "/v1/git/add"
    ) {
        return HttpResponse {
            status: 404,
            body: json!({ "error": "Not found" }).to_string(),
            content_type: JSON_CONTENT_TYPE,
        };
    }
    if method != "POST" {
        return HttpResponse {
            status: 405,
            body: json!({ "error": "Method not allowed" }).to_string(),
            content_type: JSON_CONTENT_TYPE,
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
                        content_type: JSON_CONTENT_TYPE,
                    };
                }
            };
            match crate::agent_service::run(&request.directory, request.agent, request.input) {
                Ok(result) => HttpResponse {
                    status: 200,
                    body: json!({ "result": result }).to_string(),
                    content_type: JSON_CONTENT_TYPE,
                },
                Err(error) => HttpResponse {
                    status: 500,
                    body: json!({ "error": error }).to_string(),
                    content_type: JSON_CONTENT_TYPE,
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
                        content_type: JSON_CONTENT_TYPE,
                    };
                }
            };
            match crate::git_service::status(&request.directory) {
                Ok(paths) => HttpResponse {
                    status: 200,
                    body: json!({ "paths": paths }).to_string(),
                    content_type: JSON_CONTENT_TYPE,
                },
                Err(error) => HttpResponse {
                    status: 500,
                    body: json!({ "error": error }).to_string(),
                    content_type: JSON_CONTENT_TYPE,
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
                        content_type: JSON_CONTENT_TYPE,
                    };
                }
            };
            match crate::git_service::add(&request.directory) {
                Ok(()) => HttpResponse {
                    status: 200,
                    body: json!({ "status": "ok" }).to_string(),
                    content_type: JSON_CONTENT_TYPE,
                },
                Err(error) => HttpResponse {
                    status: 500,
                    body: json!({ "error": error }).to_string(),
                    content_type: JSON_CONTENT_TYPE,
                },
            }
        }
        _ => unreachable!("the route was validated above"),
    }
}
