use super::super::{content_type::TEXT, openapi::OPENAPI};
use axum::{
    http::header,
    response::{IntoResponse, Response},
};

#[utoipa::path(
    get,
    path = "/openapi.yaml",
    operation_id = "getOpenApiDocument",
    summary = "Download the OpenAPI specification",
    description = "Returns the complete specification served by this S.I.R.K. binary as plain UTF-8 text.",
    responses(
        (status = 200, description = "The OpenAPI document.", body = String, content_type = "text/plain"),
        (status = 405, description = "The endpoint does not accept the HTTP method used.", body = super::super::schemas::responses::ErrorResponse, example = json!({"error": "Method not allowed"}))
    )
)]
pub(in crate::http) async fn openapi() -> Response {
    ([(header::CONTENT_TYPE, TEXT)], OPENAPI).into_response()
}
