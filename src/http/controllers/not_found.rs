use super::super::{json_response::JsonResponse, schemas::responses::ErrorResponse};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub(in crate::http) async fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        JsonResponse(ErrorResponse {
            error: "Not found".to_owned(),
        }),
    )
        .into_response()
}
