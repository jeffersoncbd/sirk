use super::super::{content_type::HTML, swagger::SWAGGER_UI};
use axum::{
    http::header,
    response::{IntoResponse, Response},
};

#[utoipa::path(
    get,
    path = "/swagger",
    operation_id = "getSwaggerUi",
    summary = "Open the Swagger UI",
    description = "Returns an HTML page that loads Swagger UI and configures it to display `/openapi.yaml`.",
    responses(
        (status = 200, description = "The Swagger UI HTML page.", body = String, content_type = "text/html"),
        (status = 405, description = "The endpoint does not accept the HTTP method used.", body = super::super::schemas::responses::ErrorResponse, example = json!({"error": "Method not allowed"}))
    )
)]
pub(in crate::http) async fn swagger() -> Response {
    ([(header::CONTENT_TYPE, HTML)], SWAGGER_UI).into_response()
}
