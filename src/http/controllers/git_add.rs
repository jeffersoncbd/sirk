use super::super::types::{DirectoryRequest, JsonResponse};
use axum::{
    body::Bytes,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub(in crate::http) async fn git_add(body: Bytes) -> Response {
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
    let request = match serde_json::from_str::<DirectoryRequest>(&body) {
        Ok(request) => request,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                JsonResponse(json!({ "error": "Invalid request" })),
            )
                .into_response();
        }
    };
    match tokio::task::spawn_blocking(move || crate::git_service::add(&request.directory)).await {
        Ok(Ok(())) => JsonResponse(json!({ "status": "ok" })).into_response(),
        Ok(Err(error)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(json!({ "error": error })),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(json!({ "error": format!("Git add task failed: {error}") })),
        )
            .into_response(),
    }
}
