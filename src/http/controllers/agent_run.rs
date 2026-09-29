use super::super::types::{AgentRequest, JsonResponse};
use axum::{
    body::Bytes,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub(in crate::http) async fn agent_run(body: Bytes) -> Response {
    let body = match String::from_utf8(body.to_vec()) {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                JsonResponse(json!({ "error": "Invalid request body" })),
            )
                .into_response();
        }
    };
    let request = match serde_json::from_str::<AgentRequest>(&body) {
        Ok(request) => request,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                JsonResponse(json!({ "error": "Invalid request" })),
            )
                .into_response();
        }
    };
    match tokio::task::spawn_blocking(move || {
        crate::agent_service::run(&request.directory, request.agent, request.input)
    })
    .await
    {
        Ok(Ok(result)) => JsonResponse(json!({ "result": result })).into_response(),
        Ok(Err(error)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(json!({ "error": error })),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(json!({ "error": format!("Agent task failed: {error}") })),
        )
            .into_response(),
    }
}
