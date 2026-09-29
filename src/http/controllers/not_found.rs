use super::super::types::JsonResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub(in crate::http) async fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        JsonResponse(json!({ "error": "Not found" })),
    )
        .into_response()
}
