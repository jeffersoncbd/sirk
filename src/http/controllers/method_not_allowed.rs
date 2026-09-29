use super::super::{json_response::JsonResponse, schemas::responses::ErrorResponse};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub(in crate::http) async fn method_not_allowed() -> Response {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        JsonResponse(ErrorResponse {
            error: "Method not allowed".to_owned(),
        }),
    )
        .into_response()
}
