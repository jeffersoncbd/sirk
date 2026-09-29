use super::super::types::JsonResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub(in crate::http) async fn method_not_allowed() -> Response {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        JsonResponse(json!({ "error": "Method not allowed" })),
    )
        .into_response()
}
